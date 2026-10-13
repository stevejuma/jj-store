//! Dispatch through the admitted agent API, never through a second prompt runtime.
use super::*;
use jj_store_service::agent_execution as a;
impl Host {
    async fn agent_request<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: Option<&impl serde::Serialize>,
    ) -> Result<T> {
        let endpoint = self
            .config
            .agent_endpoint
            .as_ref()
            .ok_or("Agent endpoint absent")?;
        let url = url::Url::parse(&endpoint.url)?.join(path)?;
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(35))
            .build()?;
        let mut request = if let Some(body) = body {
            client.post(url).json(body)
        } else {
            client.get(url)
        };
        if let Some(reference) = &endpoint.token_environment {
            request = request
                .bearer_auth(std::env::var(reference).map_err(|_| "Agent credential unavailable")?);
        }
        let mut response = request
            .header("x-jj-store-protocol", "1")
            .send()
            .await?
            .error_for_status()?;
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            if bytes.len() + chunk.len() > 1024 * 1024 {
                return Err("Agent response byte limit".into());
            }
            bytes.extend_from_slice(&chunk);
        }
        Ok(serde_json::from_slice(&bytes)?)
    }
    pub(super) async fn agent(
        &mut self,
        run: &w::Run,
        action: &w::Action,
        agent: &str,
        prompt: &str,
    ) -> Result<w::RunStatus> {
        let active = self
            .journal
            .active
            .clone()
            .ok_or("Workflow attempt absent")?;
        let endpoint = self
            .config
            .agent_endpoint
            .as_ref()
            .ok_or("Agent endpoint absent")?;
        let prefix = if endpoint.local {
            "v1/local/agents"
        } else {
            "v1/agents"
        };
        let request_id = m::RequestId::new(w::identity(
            b"workflow-agent",
            &[active.attempt.as_bytes(), action.id.as_bytes()],
        ));
        let source = self
            .config
            .sources
            .iter()
            .find(|s| s.repository == run.source.repository)
            .ok_or("Agent source not admitted")?;
        let send = a::conversations::Send {
            version: 1,
            request_id,
            conversation_id: m::RequestId::new(w::identity(
                b"workflow-conversation",
                &[active.attempt.as_bytes(), action.id.as_bytes()],
            ))
            .to_string(),
            origin: a::conversations::Origin {
                scope: "repository".into(),
                home: Some(self.journal.home),
                thread: None,
                agent_target: None,
                context: format!("Workflow {} action {}", run.id, action.id),
            },
            thread_codebase: None,
            agent_ids: vec![agent.into()],
            attachments: vec![a::conversations::Attachment {
                repository: run.source.repository,
                target_id: source.target_id.clone(),
                operation: run.source.operation,
                commit: run.source.commit.map(|v| v.to_string()),
            }],
            text: if let Some(context) = &run.source.conversation {
                format!(
                    "{prompt}\n\nWorkflow trigger source (immutable references): {}",
                    serde_json::to_string(context)?
                )
            } else {
                prompt.into()
            },
            comments: None,
            max_duration_ms: action.timeout_seconds * 1000,
        };
        // Persist the exact dispatch before inference. If this process is interrupted,
        // tick marks its claimed attempt unknown instead of replaying the prompt.
        let path = self
            .config
            .state_dir
            .join(format!("agent-{request_id}.json"));
        let mut file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)?;
        use std::io::Write as _;
        file.write_all(&serde_json::to_vec(&send)?)?;
        file.sync_all()?;
        let outcome: a::conversations::SendOutcome = self
            .agent_request(&format!("{prefix}/chat"), Some(&send))
            .await?;
        let initial = outcome
            .agents
            .first()
            .ok_or("Agent dispatch receipt absent")?;
        let child_request = initial.request_id;
        let deadline =
            tokio::time::Instant::now() + Duration::from_secs(action.timeout_seconds + 30);
        let mut outcome = initial.clone();
        let mut linked = false;
        let mut ticks = 0;
        loop {
            if !linked && let Some(session) = &outcome.session_id {
                self.report(w::EvidenceData::Agent {
                    session: session.clone(),
                    request_id: child_request,
                })
                .await?;
                linked = true;
            }
            match outcome.state.as_str() {
                "completed" => return Ok(w::RunStatus::Completed),
                "failed" => return Ok(w::RunStatus::Failed),
                "cancelled" => return Ok(w::RunStatus::Cancelled),
                _ => {}
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(
                    "Agent execution outcome is unknown; original dispatch retained".into(),
                );
            }
            let cancel = self.current().await?.current().status == w::RunStatus::CancelRequested;
            if cancel && let Some(session) = &outcome.session_id {
                let command = a::Command {
                    version: 1,
                    request_id: m::RequestId::new(w::identity(
                        b"workflow-agent-cancel",
                        &[request_id.as_bytes()],
                    )),
                    session_id: session.clone(),
                    action: a::Action::Cancel {
                        target: child_request.to_string(),
                    },
                };
                let _: a::Outcome = self
                    .agent_request(&format!("{prefix}/command"), Some(&command))
                    .await?;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
            ticks += 1;
            if ticks % 5 == 0 {
                self.submit(w::Command::Heartbeat {
                    run: run.id,
                    attempt: active.attempt,
                })
                .await?;
            }
            outcome = self
                .agent_request::<a::Outcome>(
                    &format!("{prefix}/outcome?request={child_request}"),
                    None::<&a::Command>,
                )
                .await?;
        }
    }
}
