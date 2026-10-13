use super::*;
use std::collections::BTreeMap;
use tokio::io::AsyncReadExt as _;

/// Retain the possible partial secret suffix across pipe chunks before publishing.
struct Redactor {
    pending: Vec<u8>,
    secrets: Vec<Vec<u8>>,
    tail: usize,
}
impl Redactor {
    fn new(mut secrets: Vec<Vec<u8>>) -> Self {
        secrets.retain(|secret| !secret.is_empty());
        secrets.sort_by_key(|secret| std::cmp::Reverse(secret.len()));
        let tail = secrets
            .iter()
            .map(Vec::len)
            .max()
            .unwrap_or(1)
            .saturating_sub(1);
        Self {
            pending: Vec::new(),
            secrets,
            tail,
        }
    }
    fn push(&mut self, bytes: &[u8], last: bool) -> String {
        self.pending.extend_from_slice(bytes);
        let limit = if last {
            self.pending.len()
        } else {
            self.pending.len().saturating_sub(self.tail)
        };
        let mut consumed = 0;
        let mut output = Vec::new();
        while consumed < limit {
            if let Some(secret) = self
                .secrets
                .iter()
                .find(|secret| self.pending[consumed..].starts_with(secret))
            {
                output.extend_from_slice(b"[redacted]");
                consumed += secret.len();
            } else {
                output.push(self.pending[consumed]);
                consumed += 1;
            }
        }
        self.pending.drain(..consumed);
        String::from_utf8_lossy(&output).replace('\0', "\\0")
    }
}
async fn pipe(
    mut pipe: impl tokio::io::AsyncRead + Unpin,
    stream: &'static str,
    sender: tokio::sync::mpsc::Sender<(&'static str, Vec<u8>)>,
) {
    let mut buffer = [0u8; 4096];
    loop {
        match pipe.read(&mut buffer).await {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                if sender.send((stream, buffer[..n].to_vec())).await.is_err() {
                    break;
                }
            }
        }
    }
}
impl Host {
    pub(super) async fn command(
        &mut self,
        run: &w::Run,
        action: &w::Action,
        executor: &config::Command,
    ) -> Result<w::RunStatus> {
        #[cfg(not(unix))]
        {
            let _ = (run, action, executor);
            return Err(
                "Command workflow execution requires the qualified Unix process profile".into(),
            );
        }
        let active = self
            .journal
            .active
            .clone()
            .ok_or("Workflow attempt absent")?;
        let parent = self.config.state_dir.join("workspaces");
        std::fs::create_dir_all(&parent)?;
        let workspace = parent.join(active.attempt.to_string());
        if !workspace.exists() {
            if let (Some(operation), Some(commit)) = (run.source.operation, run.source.commit) {
                let source = self
                    .config
                    .sources
                    .iter()
                    .find(|s| s.repository == run.source.repository)
                    .ok_or("Workflow source not admitted")?;
                let result = tokio::process::Command::new(&self.config.jj_executable)
                    .arg("--no-pager")
                    .arg("--ignore-working-copy")
                    .arg("-R")
                    .arg(&source.workspace)
                    .args(["workflow", "source", "--repo-id"])
                    .arg(run.source.repository.to_string())
                    .arg("--operation")
                    .arg(operation.to_string())
                    .arg("--commit")
                    .arg(commit.to_string())
                    .arg(&workspace)
                    .stdin(Stdio::null())
                    .output()
                    .await?;
                if !result.status.success() {
                    let reason =
                        String::from_utf8_lossy(&result.stderr[..result.stderr.len().min(4096)])
                            .into_owned();
                    self.report(w::EvidenceData::Step {
                        step: action.id.clone(),
                        status: w::RunStatus::Failed,
                        output: BTreeMap::from([("source_export".into(), reason)]),
                    })
                    .await?;
                    return Ok(w::RunStatus::Failed);
                }
            }
        } else {
            std::fs::create_dir(&workspace)?;
        }
        let operation = run
            .source
            .operation
            .map(|v| v.to_string())
            .unwrap_or_default();
        let commit = run.source.commit.map(|v| v.to_string()).unwrap_or_default();
        let mut secrets = Vec::new();
        let mut environment = executor.environment.clone();
        for (key, reference) in &executor.secrets {
            let value =
                std::env::var(reference).map_err(|_| "Workflow secret reference is unavailable")?;
            if value.is_empty() || value.len() > 1024 {
                return Err("Workflow secret reference exceeds bounds".into());
            }
            secrets.push(value.as_bytes().to_vec());
            environment.insert(key.clone(), value);
        }
        let args: Vec<_> = executor
            .args
            .iter()
            .map(|a| {
                a.replace("{workspace}", &workspace.to_string_lossy())
                    .replace("{repository}", &run.source.repository.to_string())
                    .replace("{operation}", &operation)
                    .replace("{commit}", &commit)
            })
            .collect();
        let mut command = tokio::process::Command::new(&self.config.jj_executable);
        command
            .arg(worker::MARKER)
            .arg(action.timeout_seconds.to_string())
            .arg(&executor.program)
            .args(args)
            .current_dir(&workspace)
            .env_clear()
            .envs(environment)
            .env("JJ_WORKFLOW_SOURCE", serde_json::to_string(&run.source)?)
            .env("JJ_WORKFLOW_RUN", run.id.to_string())
            .env("JJ_WORKFLOW_ATTEMPT", active.attempt.to_string())
            .env("JJ_WORKFLOW_REPOSITORY", run.source.repository.to_string())
            .env("JJ_WORKFLOW_OPERATION", &operation)
            .env("JJ_WORKFLOW_COMMIT", &commit)
            .env("JJ_WORKFLOW_INPUTS", serde_json::to_string(&action.inputs)?)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn()?;
        let mut lifetime = child.stdin.take();
        let (sender, mut receiver) = tokio::sync::mpsc::channel(16);
        let stdout = tokio::spawn(pipe(
            child.stdout.take().ok_or("Command stdout absent")?,
            "stdout",
            sender.clone(),
        ));
        let stderr = tokio::spawn(pipe(
            child.stderr.take().ok_or("Command stderr absent")?,
            "stderr",
            sender,
        ));
        let mut redactors: BTreeMap<_, _> = [
            ("stdout", Redactor::new(secrets.clone())),
            ("stderr", Redactor::new(secrets)),
        ]
        .into_iter()
        .collect();
        let deadline = tokio::time::Instant::now() + Duration::from_secs(action.timeout_seconds);
        let mut ticker = tokio::time::interval(Duration::from_secs(1));
        let mut ticks = 0;
        let mut bytes = 0usize;
        let mut result = w::RunStatus::Completed;
        loop {
            tokio::select! {
                item=receiver.recv()=>match item {
                    Some((stream,data))=> {
                        bytes+=data.len();
                        if bytes>10*1024*1024 { result=w::RunStatus::Failed;lifetime.take();break; }
                        let text=redactors.get_mut(stream).expect("pipe redactor").push(&data,false);
                        if !text.is_empty() { self.report(w::EvidenceData::Log { stream:stream.into(),text }).await?; }
                    }
                    None=>{ break; }
                },
                _=ticker.tick()=> {
                    ticks+=1;
                    if self.current().await?.current().status==w::RunStatus::CancelRequested { result=w::RunStatus::Cancelled;lifetime.take();break; }
                    if tokio::time::Instant::now()>=deadline { result=w::RunStatus::Failed;lifetime.take();break; }
                    if ticks%5==0 { self.submit(w::Command::Heartbeat { run:run.id,attempt:active.attempt }).await?; }
                }
            }
        }
        let exit = child.wait().await?;
        lifetime.take();
        stdout.abort();
        stderr.abort();
        for (stream, redactor) in &mut redactors {
            let text = redactor.push(&[], true);
            if !text.is_empty() {
                self.report(w::EvidenceData::Log {
                    stream: (*stream).into(),
                    text,
                })
                .await?;
            }
        }
        if result == w::RunStatus::Completed && !exit.success() {
            result = w::RunStatus::Failed;
        }
        if result == w::RunStatus::Completed {
            if std::fs::symlink_metadata(&workspace)?
                .file_type()
                .is_symlink()
            {
                return Err("Workflow artifact workspace symlink refused".into());
            }
            for name in &executor.artifacts {
                let mut path = workspace.clone();
                for component in name.split('/') {
                    path.push(component);
                    if std::fs::symlink_metadata(&path)?.file_type().is_symlink() {
                        return Err("Workflow artifact symlink refused".into());
                    }
                }
                let data = jj_store_server::policy::read_regular(&path, 128 * 1024)?;
                self.report(w::EvidenceData::Artifact {
                    name: name.clone(),
                    media_type: "application/octet-stream".into(),
                    bytes: data,
                })
                .await?;
            }
        }
        Ok(result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn redacts_secret_across_pipe_boundaries() {
        let mut r = Redactor::new(vec![b"abcd1234".to_vec()]);
        let mut output = r.push(b"before abcd", false);
        output += &r.push(b"1234 after", false);
        output += &r.push(&[], true);
        assert_eq!(output, "before [redacted] after");
    }
    #[test]
    fn overlapping_secrets_wait_for_the_longest_cross_chunk_match() {
        let mut redactor = Redactor::new(vec![b"token".to_vec(), b"token-secret".to_vec()]);
        let mut output = redactor.push(b"before token-", false);
        output.push_str(&redactor.push(b"secret after token", false));
        output.push_str(&redactor.push(&[], true));
        assert_eq!(output, "before [redacted] after [redacted]");
    }
    #[test]
    fn longer_secrets_and_binary_output_do_not_leak_or_reject_logs() {
        let mut r = Redactor::new(vec![b"abc".to_vec(), b"abcdef".to_vec()]);
        let mut output = r.push(b"abcdef abc\0", false);
        output += &r.push(&[], true);
        assert_eq!(output, "[redacted] [redacted]\\0");
    }
}
