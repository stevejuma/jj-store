//! First-party local YAML execution. The external runner owns Actions semantics.
use super::*;
use jj_workflow::github::{Context, Workflow};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Reporter {
    action: String,
    buffers: BTreeMap<String, String>,
    states: BTreeMap<String, (w::RunStatus, BTreeMap<String, String>)>,
    jobs: BTreeSet<String>,
    finished_jobs: BTreeSet<String>,
    secrets: Vec<String>,
}
impl Reporter {
    fn new(action: &str, secrets: Vec<String>) -> Self {
        Self {
            action: action.into(),
            buffers: BTreeMap::new(),
            states: BTreeMap::new(),
            jobs: BTreeSet::new(),
            finished_jobs: BTreeSet::new(),
            secrets,
        }
    }
    pub(super) fn feed(
        &mut self,
        stream: &str,
        text: &str,
        last: bool,
    ) -> Result<Vec<w::EvidenceData>> {
        let buffer = self.buffers.entry(stream.into()).or_default();
        buffer.push_str(text);
        if buffer.len() > 1024 * 1024 {
            return Err("Forgejo log record exceeds bounds".into());
        }
        let mut lines = Vec::new();
        while let Some(end) = buffer.find('\n') {
            lines.push(buffer.drain(..=end).collect::<String>());
        }
        if last && !buffer.is_empty() {
            lines.push(std::mem::take(buffer));
        }
        let mut reports = Vec::new();
        for line in lines {
            let event = serde_json::from_str::<Value>(line.trim_end());
            let Ok(event) = event else {
                reports.push(w::EvidenceData::Log {
                    step: Some(self.action.clone()),
                    stream: stream.into(),
                    text: line,
                });
                continue;
            };
            let job = event["jobID"].as_str();
            let mut scope = self.action.clone();
            if let Some(job) = job {
                let matrix = serde_json::to_string(&event["matrix"])?;
                let job_key =
                    hex::encode(Sha256::digest(format!("{job}:{matrix}:{}", event["job"])))[..24]
                        .to_owned();
                let job_id = format!("{}.j{job_key}", &self.action[..self.action.len().min(96)]);
                self.jobs.insert(job_id.clone());
                let ids = event["stepID"].as_array().filter(|ids| !ids.is_empty());
                let mut output = BTreeMap::from([
                    (
                        "name".into(),
                        event["job"]
                            .as_str()
                            .unwrap_or(job)
                            .split_once('/')
                            .map_or(job, |(_, name)| name)
                            .to_owned(),
                    ),
                    ("parent".into(), self.action.clone()),
                    ("kind".into(), "job".into()),
                ]);
                scope = job_id.clone();
                if let Some(ids) = ids {
                    let stage = event["stage"].as_str().unwrap_or("Main");
                    let key = format!("{job_key}:{ids:?}:{stage}");
                    scope = format!(
                        "{}.s{}",
                        &self.action[..self.action.len().min(96)],
                        &hex::encode(Sha256::digest(key))[..24]
                    );
                    let name = event["step"].as_str().unwrap_or("Step");
                    output.insert(
                        "name".into(),
                        if stage == "Main" {
                            name.into()
                        } else {
                            format!("{stage}: {name}")
                        },
                    );
                    output.insert("parent".into(), job_id.clone());
                    output.insert("kind".into(), "step".into());
                    output.insert(
                        "job".into(),
                        event["job"]
                            .as_str()
                            .unwrap_or(job)
                            .split_once('/')
                            .map_or(job, |(_, name)| name)
                            .into(),
                    );
                    output.insert("order".into(), event["stepNumber"].to_string());
                }
                for value in output.values_mut() {
                    for secret in &self.secrets {
                        *value = value.replace(secret, "[redacted]");
                    }
                }
                let outcome = if ids.is_some() {
                    event["stepResult"].as_str()
                } else {
                    event["jobResult"].as_str()
                };
                let status = match outcome {
                    Some("success") => w::RunStatus::Completed,
                    Some("failure") => w::RunStatus::Failed,
                    Some("skipped") => w::RunStatus::Skipped,
                    Some("cancelled" | "canceled") => w::RunStatus::Cancelled,
                    Some(_) => w::RunStatus::Unknown,
                    None => w::RunStatus::Running,
                };
                // Composite child outcomes never terminate the outer step.
                if ids.is_none() && outcome.is_some() {
                    self.finished_jobs.insert(job_id);
                }
                let changed = self
                    .states
                    .get(&scope)
                    .is_none_or(|(old, _)| *old != status)
                    && (outcome.is_some() || !self.states.contains_key(&scope));
                if changed {
                    self.states
                        .insert(scope.clone(), (status.clone(), output.clone()));
                    reports.push(w::EvidenceData::Step {
                        step: scope.clone(),
                        status,
                        output,
                    });
                }
            }
            if matches!(event["command"].as_str(), Some("group" | "endgroup")) {
                continue;
            }
            if let Some(message) = event["msg"].as_str() {
                let mut message = message.to_owned();
                for secret in &self.secrets {
                    message = message.replace(secret, "[redacted]");
                }
                if !message.ends_with('\n') {
                    message.push('\n');
                }
                let mut remaining = message.as_str();
                while !remaining.is_empty() {
                    let mut end = remaining.len().min(16 * 1024);
                    while !remaining.is_char_boundary(end) {
                        end -= 1;
                    }
                    reports.push(w::EvidenceData::Log {
                        step: Some(scope.clone()),
                        stream: "runner".into(),
                        text: remaining[..end].into(),
                    });
                    remaining = &remaining[end..];
                }
            }
        }
        Ok(reports)
    }
    pub(super) fn complete(&self) -> bool {
        !self.jobs.is_empty()
            && self.jobs == self.finished_jobs
            && self
                .states
                .values()
                .all(|(s, _)| *s != w::RunStatus::Unknown)
    }
    pub(super) fn close(&mut self, status: w::RunStatus) -> Vec<w::EvidenceData> {
        self.states
            .iter_mut()
            .filter_map(|(id, (current, output))| {
                if *current != w::RunStatus::Running {
                    return None;
                }
                *current = if matches!(status, w::RunStatus::Cancelled | w::RunStatus::Failed) {
                    status.clone()
                } else {
                    w::RunStatus::Unknown
                };
                Some(w::EvidenceData::Step {
                    step: id.clone(),
                    status: current.clone(),
                    output: output.clone(),
                })
            })
            .collect()
    }
}
impl Host {
    pub(super) async fn github_workflow(
        &mut self,
        run: &w::Run,
        action: &w::Action,
        path: &str,
    ) -> Result<w::RunStatus> {
        let result = self.prepare_github(run, action, path).await;
        match result {
            Ok(Some((workspace, command, reporter))) => {
                self.process(run, action, &command, &workspace, Some(reporter))
                    .await
            }
            Ok(None) => Ok(w::RunStatus::Skipped),
            Err(error) => {
                self.report(w::EvidenceData::Step {
                    step: action.id.clone(),
                    status: w::RunStatus::Failed,
                    output: BTreeMap::from([("reason".into(), error.to_string())]),
                })
                .await?;
                Ok(w::RunStatus::Failed)
            }
        }
    }
    async fn prepare_github(
        &mut self,
        run: &w::Run,
        action: &w::Action,
        path: &str,
    ) -> Result<Option<(PathBuf, config::Command, Reporter)>> {
        let runner = self
            .config
            .forgejo
            .clone()
            .ok_or("Forgejo executor not admitted")?;
        let version = tokio::process::Command::new(&runner.program)
            .arg("--version")
            .stdin(Stdio::null())
            .output()
            .await?;
        if !version.status.success()
            || !String::from_utf8_lossy(&version.stdout).contains("v13.2.0")
        {
            return Err("Forgejo Runner v13.2.0 is required by this profile".into());
        }
        let active = self.journal.active.clone().ok_or("Attempt missing")?;
        let page = self
            .client
            .workflow_page(&w::Query {
                run: Some(run.id),
                ..Default::default()
            })
            .await?;
        let event = match page.event.as_ref().map(|e| &e.kind) {
            None => "workflow_dispatch",
            Some(w::EventKind::BookmarkChanged | w::EventKind::CodePublished) => "push",
            _ => return Err("This YAML executor supports code events and manual dispatch".into()),
        };
        let source = self
            .config
            .sources
            .iter()
            .find(|s| s.repository == run.source.repository)
            .ok_or("Source not admitted")?;
        let operation = run
            .source
            .operation
            .ok_or("YAML execution requires a code operation")?;
        let commit = run
            .source
            .commit
            .ok_or("YAML execution requires a code revision")?;
        let parent = self
            .config
            .state_dir
            .join("forgejo")
            .join(active.attempt.to_string())
            .join(&action.id);
        std::fs::create_dir_all(&parent)?;
        let workspace = parent.join("source");
        let changed = parent.join("changed-paths.json");
        let mut export = tokio::process::Command::new(&self.config.jj_executable);
        export
            .args(["--no-pager", "--ignore-working-copy", "-R"])
            .arg(&source.workspace)
            .args(["workflow", "source", "--repo-id"])
            .arg(run.source.repository.to_string())
            .arg("--operation")
            .arg(operation.to_string())
            .arg("--commit")
            .arg(commit.to_string())
            .arg("--changed-paths")
            .arg(&changed);
        if let Some(context) = page.event.as_ref().and_then(|e| e.code_context.as_ref())
            && context.comparable
        {
            if let Some(before) = context.before {
                export.arg("--before").arg(before.to_string());
            } else {
                export.arg("--empty-baseline");
            }
        }
        let out = export.arg(&workspace).stdin(Stdio::null()).output().await?;
        if !out.status.success() {
            return Err("Exact source export failed".into());
        }
        let paths: Option<Vec<String>> = serde_json::from_slice(
            &jj_store_server::policy::read_regular(&changed, 8 * 1024 * 1024)?,
        )?;
        let paths = if page.event.as_ref().is_some_and(|e| {
            e.kind == w::EventKind::BookmarkChanged
                && e.code_context.as_ref().is_none_or(|c| !c.comparable)
        }) {
            None
        } else {
            paths
        };
        let file = workspace.join(path);
        if !file.exists() {
            self.report(w::EvidenceData::Step {
                step: action.id.clone(),
                status: w::RunStatus::Skipped,
                output: BTreeMap::from([(
                    "reason".into(),
                    "Workflow file absent at the recorded revision".into(),
                )]),
            })
            .await?;
            return Ok(None);
        }
        let yaml = jj_store_server::policy::read_regular(&file, w::MAX_TEXT as u64)?;
        let workflow = Workflow::parse(std::str::from_utf8(&yaml)?)?;
        if !workflow.matches(Context {
            event,
            branch: run.source.bookmark.as_deref(),
            paths: paths.as_deref(),
        })? {
            self.report(w::EvidenceData::Step {
                step: action.id.clone(),
                status: w::RunStatus::Skipped,
                output: BTreeMap::from([(
                    "reason".into(),
                    "YAML trigger filters did not match the recorded event".into(),
                )]),
            })
            .await?;
            return Ok(None);
        }
        for job in workflow.document["jobs"]
            .as_mapping()
            .ok_or("Jobs missing")?
            .values()
        {
            let value = &job["runs-on"];
            let labels = if let Some(s) = value.as_str() {
                vec![s]
            } else if let Some(items) = value.as_sequence() {
                items
                    .iter()
                    .map(|v| v.as_str().ok_or("Dynamic runs-on unsupported"))
                    .collect::<std::result::Result<Vec<_>, _>>()?
            } else {
                return Err("Dynamic runs-on unsupported".into());
            };
            if labels.is_empty()
                || labels
                    .iter()
                    .any(|label| !self.config.labels.contains(*label))
            {
                return Err("YAML platform is not admitted by this runner's labels".into());
            }
        }
        let execution = parent.join("workflow.yml");
        std::fs::write(
            &execution,
            workflow.execution_yaml(&format!("jj-store-{}-{}", active.attempt, action.id))?,
        )?;
        let empty_env = parent.join("empty.env");
        std::fs::write(&empty_env, "")?;
        let network = format!("jj-store-{}", active.attempt);
        let label = format!("jj-store.attempt={}", active.attempt);
        let mut environment = runner.environment.clone();
        let runtime_home = parent.join("runtime-home");
        std::fs::create_dir_all(&runtime_home)?;
        environment.insert("HOME".into(), runtime_home.to_string_lossy().into_owned());
        environment.insert(
            "XDG_CACHE_HOME".into(),
            runtime_home.join("cache").to_string_lossy().into_owned(),
        );
        environment
            .entry("PATH".into())
            .or_insert_with(|| "/usr/bin:/bin".into());
        let mut args = vec![
            "exec".into(),
            "--json".into(),
            "--event".into(),
            event.into(),
            "--workflows".into(),
            execution.to_string_lossy().into_owned(),
            "--directory".into(),
            workspace.to_string_lossy().into_owned(),
            "--env-file".into(),
            empty_env.to_string_lossy().into_owned(),
            "--use-gitignore=false".into(),
            "--image".into(),
            runner.image,
            "--default-actions-url".into(),
            "https://github.com".into(),
            "--network".into(),
            network,
            "--container-opts".into(),
            format!("--label={label}"),
        ];
        if !runner.architecture.is_empty() {
            args.extend(["--container-architecture".into(), runner.architecture]);
        }
        for (key, value) in [
            ("JJ_WORKFLOW_REPOSITORY", run.source.repository.to_string()),
            ("JJ_WORKFLOW_OPERATION", operation.to_string()),
            ("JJ_WORKFLOW_COMMIT", commit.to_string()),
            ("GITHUB_RUN_ID", run.id.to_string()),
            ("GITHUB_RUN_ATTEMPT", run.attempts.len().to_string()),
        ] {
            args.extend(["--env".into(), format!("{key}={value}")]);
        }
        // Local runner contexts expose the exact native ID, never a fabricated Git SHA.
        args.extend(["--env".into(), format!("SHA_REF={commit}")]);
        if run.source.bookmark.is_none() {
            args.extend([
                "--env".into(),
                format!("GITHUB_REF=refs/jj-store/commits/{commit}"),
                "--env".into(),
                format!("GITHUB_REF_NAME={commit}"),
                "--env".into(),
                "GITHUB_REF_TYPE=commit".into(),
            ]);
        }
        if let Some(branch) = &run.source.bookmark {
            args.extend([
                "--env".into(),
                format!("GITHUB_REF=refs/heads/{branch}"),
                "--env".into(),
                format!("GITHUB_REF_NAME={branch}"),
                "--env".into(),
                "GITHUB_REF_TYPE=branch".into(),
            ]);
        }
        let mut secrets = Vec::new();
        for (key, reference) in &runner.secrets {
            secrets.push(std::env::var(reference).map_err(|_| "Secret environment unavailable")?);
            args.extend(["--secret".into(), key.clone()]);
        }
        Ok(Some((
            workspace,
            config::Command {
                program: runner.program,
                args,
                environment,
                secrets: runner.secrets,
                artifacts: Vec::new(),
            },
            Reporter::new(&action.id, secrets),
        )))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn structured_logs_separate_jobs_steps_and_partial_records() {
        let mut reporter = Reporter::new("ci", vec!["secret\nvalue".into()]);
        assert!(
            reporter
                .feed("stdout", "{\"jobID\":\"a\",", false)
                .unwrap()
                .is_empty()
        );
        let a=reporter.feed("stdout","\"job\":\"Test\",\"stepID\":[\"1\"],\"stage\":\"Main\",\"step\":\"First\",\"msg\":\"secret\\nvalue\"}\n",false).unwrap();
        assert!(matches!(
            &a[0],
            w::EvidenceData::Step {
                status: w::RunStatus::Running,
                ..
            }
        ));
        assert!(
            matches!(&a[1],w::EvidenceData::Log {step:Some(_),text,..} if text=="[redacted]\n")
        );
        let b=reporter.feed("stdout","{\"jobID\":\"b\",\"stepID\":[\"1\"],\"step\":\"Second\",\"stage\":\"Main\",\"msg\":\"other\"}\n",false).unwrap();
        assert_ne!(
            serde_json::to_value(&a[1]).unwrap()["step"],
            serde_json::to_value(&b[1]).unwrap()["step"]
        );
        assert!(!reporter.complete());
    }
    #[test]
    fn explicit_skips_and_job_completion_are_required() {
        let mut r = Reporter::new("ci", vec![]);
        let rows=r.feed("stdout","{\"jobID\":\"a\",\"job\":\"Test\",\"stepID\":[\"1\"],\"stepResult\":\"skipped\"}\r\n",false).unwrap();
        assert!(matches!(
            &rows[0],
            w::EvidenceData::Step {
                status: w::RunStatus::Skipped,
                ..
            }
        ));
        assert!(!r.complete());
        r.feed(
            "stdout",
            "{\"jobID\":\"a\",\"job\":\"Test\",\"jobResult\":\"success\"}\n",
            false,
        )
        .unwrap();
        assert!(r.complete());
    }
}
