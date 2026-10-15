//! Ordinary-local forge composition; reads never snapshot or reconcile JJ heads.
mod connection;
mod restack;
mod templates;
use super::*;
use jj_forge::{self as f, engine::Local as _, journal::Device};
use jj_lib::{config::ConfigGetResultExt as _, git};
use std::process::Stdio;
use tokio::{io::AsyncReadExt as _, process::Command as Process};

pub struct LocalForge {
    access: Arc<dyn Access>,
    root: PathBuf,
    connection: f::Connection,
    credential: Option<zeroize::Zeroizing<String>>,
    account_id: Option<String>,
}
impl LocalForge {
    async fn git(&self, args: Vec<String>) -> f::Result<String> {
        let (_, repo, _, _) = read::load(&self.access, &self.root)
            .await
            .map_err(|_| f::Error::Stale)?;
        let backend = git::get_git_backend(repo.store()).map_err(|_| f::Error::Unsupported)?;
        let mut command = Process::new("git");
        command.env_clear();
        for key in [
            "PATH",
            "HOME",
            "USERPROFILE",
            "SSH_AUTH_SOCK",
            "SYSTEMROOT",
            "TMPDIR",
        ] {
            if let Some(v) = std::env::var_os(key) {
                command.env(key, v);
            }
        }
        if let Some(header) = &self.credential {
            let api = url::Url::parse(&self.connection.api_url).map_err(|_| f::Error::Invalid)?;
            let origin = if api.host_str() == Some("api.github.com") {
                String::from("https://github.com")
            } else {
                api.origin().ascii_serialization()
            };
            command
                .env("GIT_CONFIG_COUNT", "1")
                .env("GIT_CONFIG_KEY_0", format!("http.{origin}/.extraheader"))
                .env("GIT_CONFIG_VALUE_0", header.as_str());
        }
        command
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .args(["--no-pager", "--git-dir"])
            .arg(backend.git_repo_path())
            .args([
                "-c",
                "core.hooksPath=/dev/null",
                "-c",
                "core.sshCommand=ssh -oBatchMode=yes",
                "-c",
                "http.followRedirects=false",
                "-c",
                "credential.helper=",
                "-c",
                "protocol.ext.allow=never",
                "-c",
                "protocol.file.allow=always",
            ])
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        let mut child = command.spawn().map_err(|_| f::Error::State)?;
        let mut stdout = child.stdout.take().ok_or(f::Error::State)?;
        let mut bytes = vec![];
        let result = tokio::time::timeout(std::time::Duration::from_secs(60), async {
            (&mut stdout)
                .take(1024 * 1024 + 1)
                .read_to_end(&mut bytes)
                .await
                .map_err(|_| f::Error::Unknown)?;
            let status = child.wait().await.map_err(|_| f::Error::Unknown)?;
            if bytes.len() > 1024 * 1024 {
                return Err(f::Error::Limit);
            }
            if !status.success() {
                return Err(f::Error::Unknown);
            }
            String::from_utf8(bytes).map_err(|_| f::Error::Invalid)
        })
        .await
        .map_err(|_| f::Error::Unknown)?;
        self.access
            .authorize()
            .await
            .map_err(|_| f::Error::Denied)?;
        result
    }
    async fn verify_remote(&self) -> f::Result<String> {
        let value = self
            .git(vec![
                "remote".into(),
                "get-url".into(),
                "--push".into(),
                self.connection.remote.clone(),
            ])
            .await?;
        let value = value.trim();
        // The configured remote is the approval boundary. Never accept embedded HTTP secrets.
        if value.starts_with("https://") {
            let u = url::Url::parse(value).map_err(|_| f::Error::Invalid)?;
            if !u.username().is_empty()
                || u.password().is_some()
                || u.query().is_some()
                || u.fragment().is_some()
            {
                return Err(f::Error::Invalid);
            }
            let api = url::Url::parse(&self.connection.api_url).map_err(|_| f::Error::Invalid)?;
            let host = u.host_str().ok_or(f::Error::Invalid)?;
            if api.host_str() != Some(host)
                && !(host == "github.com" && api.host_str() == Some("api.github.com"))
            {
                return Err(f::Error::Stale);
            }
            if u.port_or_known_default() != api.port_or_known_default() {
                return Err(f::Error::Stale);
            }
            if u.path().trim_start_matches('/').trim_end_matches(".git")
                != self.connection.source_repository
            {
                return Err(f::Error::Stale);
            }
        } else if value.starts_with("ssh://") {
            let u = url::Url::parse(value).map_err(|_| f::Error::Invalid)?;
            let api = url::Url::parse(&self.connection.api_url).map_err(|_| f::Error::Invalid)?;
            if u.password().is_some()
                || u.query().is_some()
                || u.fragment().is_some()
                || u.host_str() != api.host_str()
                    && !(u.host_str() == Some("github.com")
                        && api.host_str() == Some("api.github.com"))
                || u.path().trim_start_matches('/').trim_end_matches(".git")
                    != self.connection.source_repository
            {
                return Err(f::Error::Stale);
            }
        } else if let Some(address) = value.strip_prefix("git@") {
            let (host, path) = address.split_once(':').ok_or(f::Error::Invalid)?;
            let api = url::Url::parse(&self.connection.api_url).map_err(|_| f::Error::Invalid)?;
            if api.host_str() != Some(host)
                && !(host == "github.com" && api.host_str() == Some("api.github.com"))
            {
                return Err(f::Error::Stale);
            }
            if path.trim_end_matches(".git") != self.connection.source_repository {
                return Err(f::Error::Stale);
            }
        } else if !std::path::Path::new(value).is_absolute() {
            return Err(f::Error::Unsupported);
        }
        Ok(value.to_string())
    }
    pub async fn segments(&self, bookmark: &str) -> f::Result<(String, Vec<f::Segment>)> {
        self.collect(bookmark, false).await
    }
    async fn collect(
        &self,
        bookmark: &str,
        allow_target_drift: bool,
    ) -> f::Result<(String, Vec<f::Segment>)> {
        let (workspace, repo, config, settings) = read::load(&self.access, &self.root)
            .await
            .map_err(|_| f::Error::Stale)?;
        let target = config
            .active()
            .map(|w| w.target.as_str())
            .filter(|t| t.split('@').next() == Some(self.connection.target.as_str()))
            .unwrap_or(&self.connection.target);
        let target_head = resolve(repo.as_ref(), target)
            .map_err(|_| f::Error::Invalid)?
            .id()
            .hex();
        let topology = topology::discover(&repo, target, &[])
            .await
            .map_err(|_| f::Error::Stale)?;
        if !topology.complete {
            return Err(f::Error::Stale);
        }
        let (lane, index) = topology
            .inventory
            .iter()
            .find_map(|l| {
                l.heads
                    .iter()
                    .position(|h| h.names.iter().any(|n| n == bookmark))
                    .map(|i| (l, i))
            })
            .ok_or(f::Error::Invalid)?;
        let mut out = vec![];
        let heads: Vec<_> = lane.heads[index..].iter().rev().collect();
        let mut base = self.connection.target.clone();
        for head in heads {
            let branch = if head.names.iter().any(|n| n == bookmark) {
                bookmark.to_string()
            } else if head.names.len() == 1 {
                head.names[0].clone()
            } else {
                return Err(f::Error::Invalid);
            };
            let boundary = head.base_commit.as_ref().ok_or(f::Error::Invalid)?;
            let tip = resolve(repo.as_ref(), &branch).map_err(|_| f::Error::Invalid)?;
            let mut commit = tip.clone();
            let mut descriptions = vec![];
            let private = settings
                .get_string("git.private-commits")
                .optional()
                .map_err(|_| f::Error::Invalid)?
                .unwrap_or_default();
            loop {
                if commit.id().hex() == *boundary {
                    break;
                }
                if descriptions.len() >= 500 || commit.parent_ids().len() != 1 {
                    return Err(f::Error::Invalid);
                }
                if commit.has_conflict() {
                    return Err(f::Error::Invalid);
                }
                if !private.is_empty() {
                    let expression = policy::expression(
                        repo.as_ref(),
                        workspace.workspace_name(),
                        &settings,
                        &private,
                    )
                    .map_err(|_| f::Error::Invalid)?;
                    let revset = expression
                        .evaluate(repo.as_ref())
                        .map_err(|_| f::Error::Invalid)?;
                    if revset.containing_fn()(commit.id())
                        .await
                        .map_err(|_| f::Error::Invalid)?
                    {
                        return Err(f::Error::Denied);
                    }
                }
                descriptions.push(commit.description().to_string());
                commit = repo
                    .store()
                    .get_commit(&commit.parent_ids()[0])
                    .map_err(|_| f::Error::Stale)?;
            }
            if descriptions.is_empty()
                || tip.tree().tree_ids_and_labels() == commit.tree().tree_ids_and_labels()
            {
                return Err(f::Error::Invalid);
            }
            descriptions.reverse();
            let title = descriptions
                .first()
                .and_then(|s| s.lines().next())
                .filter(|s| !s.trim().is_empty())
                .unwrap_or(&branch)
                .to_string();
            out.push(f::Segment {
                branch: branch.clone(),
                head: tip.id().hex(),
                base: base.clone(),
                title,
                body: templates::body(
                    &self.root,
                    self.connection.provider,
                    descriptions.join("\n\n"),
                )?,
                remote_head: None,
                shared: head.shared,
            });
            base = branch;
        }
        let operation = repo.op_id().hex();
        drop(repo);
        self.verify_remote().await?;
        if !allow_target_drift && restack::target_head(self).await? != target_head {
            return Err(f::Error::Stale);
        }
        for s in &mut out {
            s.remote_head = self.remote_head(&s.branch).await?;
        }
        let session = ReadSession::open(self.access.clone(), self.root.clone())
            .await
            .map_err(|_| f::Error::Stale)?;
        if session.operation().to_string() != operation {
            return Err(f::Error::Stale);
        }
        Ok((operation, out))
    }
}
#[async_trait::async_trait(?Send)]
impl f::engine::Local for LocalForge {
    async fn authorize(&self) -> f::Result<()> {
        self.access.authorize().await.map_err(|_| f::Error::Denied)
    }
    async fn validate(&self, plan: &f::Plan, resuming: bool) -> f::Result<()> {
        if plan.account_id != self.account_id {
            return Err(f::Error::Stale);
        }
        let read = ReadSession::open(self.access.clone(), self.root.clone())
            .await
            .map_err(|_| f::Error::Stale)?;
        if !resuming && read.operation().to_string() != plan.operation {
            return Err(f::Error::Stale);
        }
        for item in &plan.items {
            if read.bookmark(&item.segment.branch).ok().as_deref() != Some(&item.segment.head) {
                return Err(f::Error::Stale);
            }
        }
        let bookmark = &plan.items.last().ok_or(f::Error::Invalid)?.segment.branch;
        let (_, segments) = self.collect(bookmark, resuming).await?;
        if segments.len() != plan.items.len()
            || segments.iter().zip(&plan.items).any(|(s, i)| {
                s.branch != i.segment.branch
                    || s.head != i.segment.head
                    || (!resuming && s.remote_head != i.segment.remote_head)
                    || (if plan.cumulative {
                        &plan.connection.target
                    } else {
                        &s.base
                    }) != &i.segment.base
            })
        {
            return Err(f::Error::Stale);
        }
        self.verify_remote().await.map(|_| ())
    }
    async fn remote_head(&self, branch: &str) -> f::Result<Option<String>> {
        if !f::safe_name(branch) {
            return Err(f::Error::Invalid);
        }
        let destination = self.verify_remote().await?;
        let out = self
            .git(vec![
                "ls-remote".into(),
                "--heads".into(),
                destination,
                format!("refs/heads/{branch}"),
            ])
            .await?;
        let Some(line) = out.lines().next() else {
            return Ok(None);
        };
        let (sha, name) = line.split_once('\t').ok_or(f::Error::Invalid)?;
        if sha.len() != 40
            || !sha.bytes().all(|b| b.is_ascii_hexdigit())
            || name != format!("refs/heads/{branch}")
        {
            return Err(f::Error::Invalid);
        }
        Ok(Some(sha.into()))
    }
    async fn push(&self, s: &f::Segment) -> f::Result<()> {
        let read = ReadSession::open(self.access.clone(), self.root.clone())
            .await
            .map_err(|_| f::Error::Stale)?;
        if read.bookmark(&s.branch).ok().as_deref() != Some(&s.head) {
            return Err(f::Error::Stale);
        }
        let destination = self.verify_remote().await?;
        if self.remote_head(&s.branch).await? != s.remote_head {
            return Err(f::Error::Stale);
        }
        self.git(vec![
            "push".into(),
            "--receive-pack=git-receive-pack".into(),
            "--porcelain".into(),
            "--no-verify".into(),
            format!(
                "--force-with-lease=refs/heads/{}:{}",
                s.branch,
                s.remote_head.as_deref().unwrap_or("")
            ),
            destination,
            format!("{}:refs/heads/{}", s.head, s.branch),
        ])
        .await
        .map_err(|_| f::Error::Unknown)?;
        if self.remote_head(&s.branch).await?.as_deref() != Some(&s.head) {
            return Err(f::Error::Unknown);
        }
        Ok(())
    }
}
pub async fn handle(
    access: Arc<dyn Access>,
    root: PathBuf,
    mut request: f::Request,
) -> f::Result<f::Response> {
    if request.version != 1 {
        return Err(f::Error::Invalid);
    }
    access.authorize().await.map_err(|_| f::Error::Denied)?;
    let metadata = access
        .metadata_path()
        .canonicalize()
        .map_err(|_| f::Error::State)?;
    if std::fs::read_to_string(metadata.join("store/type"))
        .ok()
        .as_deref()
        .map(str::trim)
        != Some("git")
    {
        return Err(f::Error::Unsupported);
    }
    let mut device = Device::open(&metadata)?;
    if let f::Action::Detect { auto_connect } = &request.action {
        let local = LocalForge {
            access,
            root,
            connection: device.connection().unwrap_or_else(connection::provisional),
            credential: None,
            account_id: None,
        };
        let detection = connection::detect(&local, &mut device, *auto_connect).await?;
        return Ok(f::Response {
            connection: device.connection(),
            detection: Some(detection),
            ..Default::default()
        });
    }
    if let f::Action::BindAccount {
        account,
        connection: selected,
    } = &request.action
    {
        let local = LocalForge {
            access,
            root,
            connection: selected.clone(),
            credential: None,
            account_id: None,
        };
        connection::bind(
            &local,
            f::accounts::native()?,
            &mut device,
            account.clone(),
            selected.clone(),
        )
        .await?;
        return Ok(f::Response {
            connection: device.connection(),
            detection: Some(connection::detect(&local, &mut device, false).await?),
            ..Default::default()
        });
    }
    if let f::Action::Configure { connection } = request.action {
        device.configure(connection)?;
        return Ok(f::Response {
            connection: device.connection(),
            ..Default::default()
        });
    }
    let Some(connection) = device.connection() else {
        return Ok(f::Response::default());
    };
    let supplied = request
        .token
        .take()
        .or_else(|| std::env::var("JJ_STORE_FORGE_TOKEN").ok());
    let account_id = request.account_id.clone().or_else(|| {
        if supplied.is_none() {
            device.account_id()
        } else {
            None
        }
    });
    let token = if let Some(id) = &account_id {
        if device.account_id().as_ref() != Some(id) {
            return Err(f::Error::Stale);
        }
        let manager = f::accounts::native()?;
        if manager.account(id)?.api_url != connection.api_url {
            return Err(f::Error::Denied);
        }
        manager
            .credential(id, f::accounts::now()?)
            .await?
            .access_token
            .clone()
    } else {
        supplied.unwrap_or_default()
    };
    let secret = zeroize::Zeroizing::new(token);
    let client = f::Client::new(connection.clone(), secret.to_string())?;
    let credential = if secret.is_empty() {
        None
    } else {
        use base64::Engine as _;
        let user = client.git_username().await?;
        let raw = zeroize::Zeroizing::new(format!("{user}:{}", secret.as_str()));
        Some(zeroize::Zeroizing::new(format!(
            "Authorization: Basic {}",
            base64::engine::general_purpose::STANDARD.encode(raw.as_bytes())
        )))
    };
    let local = LocalForge {
        access: access.clone(),
        root,
        connection: connection.clone(),
        credential,
        account_id,
    };
    let mut response = f::Response {
        connection: Some(connection.clone()),
        capabilities: client.capabilities_checked().await?,
        ..Default::default()
    };
    match request.action {
        f::Action::RestackPreview { bookmark } => {
            response.restack = Some(restack::preview(&local, &client, &device, &bookmark).await?);
        }
        f::Action::Restack { plan } => {
            let (restacked, receipt) = restack::apply(&local, &client, &mut device, plan).await?;
            response.restack = Some(restacked);
            response.receipt = receipt;
        }
        f::Action::Status => {
            response.receipt = device
                .receipts()
                .into_iter()
                .rev()
                .find(|r| !matches!(r.outcome, f::Outcome::Complete | f::Outcome::Rejected));
            response.restack = device.pending_restack();
            let prs = f::engine::observe(&client, &device).await?;
            if let Err(error) = f::engine::maintain(&local, &client, &mut device, &prs).await {
                response.notice = Some(error.to_string());
            }
            response.operation = device.unresolved_operation(None);
            response.merges = device.merge_results();
            response.pull_requests = prs;
        }
        f::Action::Preview { bookmark, draft } => {
            let (op, segments) = local.segments(&bookmark).await?;
            let mut prs = f::engine::observe(&client, &device).await?;
            for segment in &segments {
                if !prs.iter().any(|p| p.head == segment.branch) {
                    prs.extend(client.find_by_head(&segment.branch).await?);
                }
            }
            let id = format!(
                "{:x}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|_| f::Error::State)?
                    .as_nanos()
            );
            response.plan = Some(f::planner::plan(id, op, connection, segments, &prs, draft)?);
            if let Some(plan) = response.plan.as_mut() {
                plan.account_id = local.account_id.clone();
            }
        }
        f::Action::Submit { plan } => {
            response.receipt =
                Some(f::engine::submit(&local, &client, &mut device, plan, false).await?);
        }
        f::Action::Recover { request_id } => {
            if let Some(old) = device.receipt(&request_id) {
                response.receipt =
                    Some(f::engine::submit(&local, &client, &mut device, old.plan, true).await?);
            } else if let Some(mut old) = device.restack(&request_id) {
                if old.operation.is_none() {
                    old.outcome = f::Outcome::Unknown;
                    response.restack = Some(old);
                } else {
                    let (restacked, receipt) =
                        restack::apply(&local, &client, &mut device, old.plan).await?;
                    response.restack = Some(restacked);
                    response.receipt = receipt;
                }
            } else {
                return Err(f::Error::Invalid);
            }
        }
        f::Action::OperationStatus { request_id } => {
            let mut operation = device.operation(&request_id).ok_or(f::Error::Invalid)?;
            if matches!(operation.outcome, f::Outcome::Running | f::Outcome::Unknown) {
                let current = client.pull_request(operation.number).await?;
                let satisfied = match &operation.action {
                    f::PrAction::Edit { title, body, .. } => {
                        current.title == *title && current.body == *body
                    }
                    f::PrAction::Draft { draft } => current.draft == *draft,
                    f::PrAction::Merge { .. } => current.state == f::PrState::Merged,
                    _ => false,
                };
                if satisfied {
                    device.finish_operation(&operation.id, Ok(None))?;
                    operation.outcome = f::Outcome::Complete;
                } else {
                    operation.outcome = f::Outcome::Unknown;
                }
            }
            response.operation = Some(operation);
        }
        f::Action::PullRequest { number } => response
            .pull_requests
            .push(client.pull_request(number).await?),
        f::Action::Pr {
            request_id,
            number,
            action,
        } => {
            let action = match action {
                f::PrAction::Edit {
                    title,
                    body,
                    expected_body,
                } => f::PrAction::Edit {
                    title,
                    body: f::planner::edit_body(&expected_body, &body)?,
                    expected_body,
                },
                action => action,
            };
            let caps = &response.capabilities;
            if matches!(&action, f::PrAction::Logs { .. }) && !caps.logs
                || matches!(&action, f::PrAction::Cancel { .. }) && !caps.cancel
                || matches!(&action, f::PrAction::Rerun { .. }) && !caps.rerun
                || matches!(&action, f::PrAction::Merge { method, .. } if !caps.merge_methods.contains(method))
            {
                return Err(f::Error::Unsupported);
            }
            let pr = client.pull_request(number).await?;
            response.pull_requests.push(pr.clone());
            match action.clone() {
                f::PrAction::Checks => response.checks = Some(client.checks(&pr).await?),
                f::PrAction::Jobs { run } => response.checks = Some(client.jobs(&pr, &run).await?),
                f::PrAction::Logs { run, job } => {
                    let jobs = client.jobs(&pr, &run).await?;
                    if !jobs
                        .checks
                        .iter()
                        .any(|j| j.log_id.as_deref() == Some(&job))
                    {
                        return Err(f::Error::Stale);
                    }
                    response.log = Some(client.logs(&job).await?);
                }
                f::PrAction::MergeStatus { receipt } => {
                    let result = client.merge_status(number, &receipt).await?;
                    device.record_merge_status(number, &receipt, result.clone())?;
                    response.merge = Some(result)
                }
                _ => {
                    if let Some(operation) = device.begin_operation(&request_id, number, &action)? {
                        if operation.outcome != f::Outcome::Complete {
                            return Err(f::Error::Unknown);
                        }
                        response.merge = operation.merge.clone();
                        response.operation = Some(operation);
                        return Ok(response);
                    }
                    access.authorize().await.map_err(|_| f::Error::Denied)?;
                    let result = async {
                        match action {
                            f::PrAction::Edit {
                                title,
                                body,
                                expected_body,
                            } => {
                                if title.trim().is_empty()
                                    || title.contains('\n')
                                    || body.len() > 128 * 1024
                                    || pr.body != expected_body
                                {
                                    Err(f::Error::Stale)
                                } else {
                                    let body = f::planner::edit_body(&pr.body, &body)?;
                                    client
                                        .update(number, Some(&title), Some(&body), None)
                                        .await
                                        .map(|_| None)
                                }
                            }
                            f::PrAction::Draft { draft } => {
                                client.draft(number, draft).await.map(|_| None)
                            }
                            f::PrAction::Merge {
                                method,
                                expected_sha,
                            } => {
                                let prs = f::engine::observe(&client, &device).await?;
                                if pr.sha != expected_sha
                                    || !f::engine::merge_dependencies_met(
                                        &pr,
                                        &connection,
                                        &device.receipts(),
                                        &prs,
                                    )
                                {
                                    Err(f::Error::Stale)
                                } else {
                                    client.merge(&pr, &method).await.map(Some)
                                }
                            }
                            f::PrAction::Rerun { run, failed_only } => {
                                client.verify_run(&pr, &run).await?;
                                client
                                    .control_run(&run, false, failed_only)
                                    .await
                                    .map(|_| None)
                            }
                            f::PrAction::Cancel { run } => {
                                client.verify_run(&pr, &run).await?;
                                client.control_run(&run, true, false).await.map(|_| None)
                            }
                            _ => Err(f::Error::Invalid),
                        }
                    }
                    .await;
                    response.merge = device.finish_operation(&request_id, result)?;
                    response.pull_requests = vec![client.pull_request(number).await?];
                }
            }
        }
        f::Action::Configure { .. } | f::Action::Detect { .. } | f::Action::BindAccount { .. } => {
            unreachable!()
        }
    }
    access.authorize().await.map_err(|_| f::Error::Denied)?;
    Ok(response)
}

#[cfg(test)]
mod tests;
