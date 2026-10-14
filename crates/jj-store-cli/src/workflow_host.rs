//! Optional execution-only host. Core repository publication does not depend on it.
mod agent;
mod command;
mod config;
mod discovery;
mod events;
mod forgejo;
mod journal;
mod managed;
mod worker;
pub use config::Config;
pub use discovery::Discovery;
use jj_store_model as m;
use jj_store_rpc::client::NativeClient;
use jj_store_service::workflow as w;
use journal::{Active, Journal};
pub use managed::ManagedRuntime;
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};
pub use worker::entry as worker_entry;
#[doc(hidden)]
pub fn worker_entry_marker() -> &'static str {
    worker::MARKER
}
type Result<T> = jj_store_jj::operation_stores::AuthorityResult<T>;
pub struct Host {
    config: Config,
    journal: Journal,
    client: NativeClient,
    _lock: std::fs::File,
}
#[derive(Debug, clap::Args)]
pub struct Args {
    #[arg(long)]
    pub config: PathBuf,
    /// Poll once, including execution of selected pending work.
    #[arg(long)]
    pub once: bool,
    #[arg(long)]
    /// Validate operator JSON and attachments without starting the host.
    pub check: bool,
}
pub async fn run(args: Args) -> Result<()> {
    let config = Config::load(&args.config)?;
    if args.check {
        return Ok(());
    }
    let mut host = Host::open(config).await?;
    loop {
        let result = host.tick().await;
        if args.once {
            return result;
        }
        if result.is_err() {
            eprintln!(
                "Workflow host paused this poll; retained request and execution identities for reconciliation"
            );
        }
        tokio::time::sleep(Duration::from_millis(host.config.poll_millis)).await;
    }
}
impl Host {
    async fn open(config: Config) -> Result<Self> {
        std::fs::create_dir_all(&config.state_dir)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            std::fs::set_permissions(&config.state_dir, std::fs::Permissions::from_mode(0o700))?;
        }
        let _lock = jj_store_server::policy::exclusive_lock(&config.state_dir.join("host.lock"))?;
        let connection = config::connection(&config.home)?;
        let client = connection.connect().await?;
        let path = config.state_dir.join("journal.json");
        let journal = if path.exists() {
            serde_json::from_slice::<Journal>(&jj_store_server::policy::read_regular(
                &path,
                8 * 1024 * 1024,
            )?)?
        } else {
            Journal {
                version: 1,
                host: config.host.clone(),
                principal: config.principal,
                home: connection.repository,
                generation: {
                    let mut bytes = [0; 16];
                    getrandom::fill(&mut bytes)?;
                    m::RequestId::new(bytes)
                },
                counter: 0,
                cursors: Default::default(),
                pending: None,
                active: None,
            }
        };
        if journal.version != 1
            || journal.host != config.host
            || journal.principal != config.principal
            || journal.home != connection.repository
        {
            return Err("Workflow journal belongs to another host or authority".into());
        }
        let host = Host {
            config,
            journal,
            client,
            _lock,
        };
        host.save()?;
        Ok(host)
    }

    // The CLI is the reporting transport. A failed/unknown response is reconciled
    // against the same receipt; no process or prompt is relaunched here.
    async fn flush(&mut self) -> Result<w::Receipt> {
        let request = self
            .journal
            .pending
            .clone()
            .ok_or("No pending workflow request")?;
        let status = self
            .client
            .workflow_command_status(request.request_id)
            .await?;
        if let w::Status::Committed { receipt } = status {
            self.accept(&request, &receipt)?;
            return Ok(*receipt);
        }
        if matches!(status, w::Status::OutcomeUnknown { .. }) {
            return Err("Workflow publication outcome is unknown".into());
        }
        let path = self.config.state_dir.join("pending.json");
        std::fs::write(&path, serde_json::to_vec(&request.command)?)?;
        let output = tokio::process::Command::new(&self.config.jj_executable)
            .arg("--no-pager")
            .arg("--ignore-working-copy")
            .arg("-R")
            .arg(&self.config.home)
            .args(["workflow", "--format", "json", "command", "--request-id"])
            .arg(request.request_id.to_string())
            .arg(&path)
            .stdin(Stdio::null())
            .output()
            .await?;
        if output.stdout.len() > 8 * 1024 * 1024 {
            return Err("Workflow CLI response limit".into());
        }
        if output.status.success()
            && let w::Outcome::Committed { receipt } =
                serde_json::from_slice::<w::Outcome>(&output.stdout)?
        {
            self.accept(&request, &receipt)?;
            return Ok(*receipt);
        }
        if let w::Status::Committed { receipt } = self
            .client
            .workflow_command_status(request.request_id)
            .await?
        {
            self.accept(&request, &receipt)?;
            return Ok(*receipt);
        }
        Err(
            "Workflow command was refused or its outcome is unknown; original request retained"
                .into(),
        )
    }
    fn accept(&mut self, request: &w::Request, receipt: &w::Receipt) -> Result<()> {
        if let w::Command::Claim { run, attempt, .. } = request.command
            && let Some(selected) = receipt.result.runs.first()
            && selected.current().owner == Some(self.config.principal)
            && selected.current().host.as_deref() == Some(self.config.host.as_str())
        {
            self.journal.active = Some(Active {
                run,
                attempt,
                sequence: selected.current().sequence,
            });
        }
        self.journal.pending = None;
        self.save()
    }
    async fn submit(&mut self, command: w::Command) -> Result<w::Receipt> {
        self.request(command)?;
        self.flush().await
    }
    async fn recover(&mut self) -> Result<()> {
        if self.journal.pending.is_some() {
            self.flush().await?;
        }
        if let Some(active) = self.journal.active.clone() {
            let page = self
                .client
                .workflow_page(&w::Query {
                    run: Some(active.run),
                    ..Default::default()
                })
                .await?;
            if let Some(run) = page.runs.first()
                && run.current().id == active.attempt
                && !run.current().status.terminal()
            {
                self.submit(w::Command::Interrupt {
                    run: active.run,
                    attempt: active.attempt,
                })
                .await?;
            }
            self.journal.active = None;
            self.save()?;
        }
        Ok(())
    }
    async fn tick(&mut self) -> Result<()> {
        self.recover().await?;
        self.observe_sources().await?;
        let mut after = None;
        loop {
            let page = self
                .client
                .workflow_page(&w::Query {
                    limit: Some(256),
                    pending_only: Some(true),
                    after,
                    ..Default::default()
                })
                .await?;
            let next = page.next;
            for run in page.runs {
                if run.current().status != w::RunStatus::Pending {
                    continue;
                }
                let detail = self
                    .client
                    .workflow_page(&w::Query {
                        run: Some(run.id),
                        ..Default::default()
                    })
                    .await?;
                let Some(definition) = detail.workflows.first() else {
                    continue;
                };
                if !self.config.eligible(definition) {
                    continue;
                }
                let claim = w::Command::Claim {
                    run: run.id,
                    attempt: run.current().id,
                    host: self.config.host.clone(),
                };
                let receipt = match self.submit(claim).await {
                    Ok(receipt) => receipt,
                    Err(error) => {
                        let detail = self
                            .client
                            .workflow_page(&w::Query {
                                run: Some(run.id),
                                ..Default::default()
                            })
                            .await?;
                        if detail.runs.first().is_some_and(|r| {
                            r.current().id != run.current().id
                                || r.current().owner.is_some_and(|owner| {
                                    owner != self.config.principal
                                        || r.current().host.as_deref()
                                            != Some(self.config.host.as_str())
                                })
                                || r.current().status.terminal()
                        }) {
                            self.journal.pending = None;
                            self.save()?;
                            continue;
                        }
                        return Err(error);
                    }
                };
                if receipt.result.runs.first().is_none_or(|r| {
                    r.current().owner != Some(self.config.principal)
                        || r.current().host.as_deref() != Some(self.config.host.as_str())
                }) {
                    return Err("Workflow host credential principal differs".into());
                }
                self.journal.active = Some(Active {
                    run: run.id,
                    attempt: run.current().id,
                    sequence: run.current().sequence,
                });
                self.save()?;
                self.execute(&run, definition).await?;
            }
            after = next;
            if after.is_none() {
                break;
            }
        }
        Ok(())
    }
    async fn report(&mut self, data: w::EvidenceData) -> Result<()> {
        let active = self
            .journal
            .active
            .clone()
            .ok_or("Workflow attempt absent")?;
        self.submit(w::Command::Report {
            run: active.run,
            attempt: active.attempt,
            sequence: active.sequence,
            data,
        })
        .await?;
        self.journal
            .active
            .as_mut()
            .expect("active attempt")
            .sequence += 1;
        self.save()
    }
    async fn current(&self) -> Result<w::Run> {
        let active = self
            .journal
            .active
            .as_ref()
            .ok_or("Workflow attempt absent")?;
        let page = self
            .client
            .workflow_page(&w::Query {
                run: Some(active.run),
                ..Default::default()
            })
            .await?;
        page.runs
            .into_iter()
            .next()
            .ok_or_else(|| "Workflow run unavailable".into())
    }
    async fn finish(&mut self, status: w::RunStatus, result: &str) -> Result<()> {
        let active = self
            .journal
            .active
            .clone()
            .ok_or("Workflow attempt absent")?;
        self.submit(w::Command::Finish {
            run: active.run,
            attempt: active.attempt,
            status,
            result: result.into(),
        })
        .await?;
        self.journal.active = None;
        self.save()
    }
    async fn execute(&mut self, run: &w::Run, definition: &w::DefinitionVersion) -> Result<()> {
        let mut status = w::RunStatus::Skipped;
        for action in &definition.definition.actions {
            if self.current().await?.current().status == w::RunStatus::CancelRequested {
                status = w::RunStatus::Cancelled;
                break;
            }
            self.report(w::EvidenceData::Step {
                step: action.id.clone(),
                status: w::RunStatus::Running,
                output: Default::default(),
            })
            .await?;
            let result = match &action.executor {
                w::Executor::Command { name } => {
                    self.command(run, action, &self.config.commands[name].clone())
                        .await?
                }
                w::Executor::GithubWorkflow { path } => {
                    self.github_workflow(run, action, path).await?
                }
                w::Executor::Agent { agent, prompt } => {
                    self.agent(run, action, agent, prompt).await?
                }
            };
            self.report(w::EvidenceData::Step {
                step: action.id.clone(),
                status: result.clone(),
                output: Default::default(),
            })
            .await?;
            if result == w::RunStatus::Completed {
                status = w::RunStatus::Completed;
            }
            if !matches!(result, w::RunStatus::Completed | w::RunStatus::Skipped) {
                status = result;
                break;
            }
        }
        if self.current().await?.current().status == w::RunStatus::CancelRequested {
            status = w::RunStatus::Cancelled;
        }
        self.finish(status, "Ordered actions finished").await
    }
}
