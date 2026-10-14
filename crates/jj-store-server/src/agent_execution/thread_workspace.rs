use super::*;
use jj_collab::{ThreadCommandV1, ThreadId};
use jj_store_service::agent_execution::thread_workspace as api;
use jj_store_service::collab_mutations::{CollabCommandV1, Request};

impl Supervisor {
    pub(super) async fn configure_thread(
        &self,
        actor: PrincipalId,
        input: api::Request,
    ) -> Result<api::Outcome> {
        self.actor(actor)?;
        if input.version != 1 {
            return Err(Error::Invalid);
        }
        // Serialize membership publication with conversation admission. A queued
        // frame is checked again at the physical dispatch boundary.
        let _dispatch = self.0.conversation_dispatch.lock().await;
        let _mutation = self.0.mutation.lock().await;
        let (selected, repositories) = match &input.command {
            api::Command::Create { repositories, .. } => (None, repositories),
            api::Command::Codebase {
                home, repositories, ..
            } => (Some(*home), repositories),
        };
        jj_collab::validate_codebase(repositories).map_err(|_| Error::Invalid)?;
        for repository in repositories {
            let profile = if self
                .0
                .resources
                .folders
                .as_ref()
                .is_some_and(|f| f.open(*repository).is_ok())
            {
                "local_jj_git"
            } else {
                "native"
            };
            self.authorized(*repository, profile, false).await?;
        }
        let (home, profile, _) = self.conversation_home(selected).await?;
        let intent = match &input.command {
            api::Command::Create {
                title,
                repositories,
            } => ThreadCommandV1::CreateIntentional {
                thread_id: ThreadId::new(*input.request_id.as_bytes()),
                title: title.trim().into(),
                repositories: repositories.clone(),
            },
            api::Command::Codebase {
                thread,
                expected,
                repositories,
                ..
            } => ThreadCommandV1::SetCodebase {
                thread_id: *thread,
                expected: *expected,
                repositories: repositories.clone(),
            },
        };
        let request = Request {
            version: 1,
            request_id: input.request_id,
            command: CollabCommandV1::Thread { intent },
        };
        let saved = self.read(|s| Ok(s.thread_requests.get(&input.request_id).cloned()))?;
        if saved
            .as_ref()
            .is_some_and(|old| old != &(home, profile.clone(), request.clone()))
        {
            return Err(Error::Conflict);
        }
        if self
            .home_thread_command_not_known(home, &profile, input.request_id)
            .await?
            && let api::Command::Codebase {
                thread,
                repositories,
                ..
            } = &input.command
        {
            let old = self.home_thread_summary(home, &profile, *thread).await?;
            let removed = removes_repository(old.value.codebase.as_deref(), home, repositories);
            if removed && self.thread_has_pending_work(home, *thread)? {
                return Err(Error::Busy);
            }
        }
        self.update(|s| {
            if !s.thread_requests.contains_key(&input.request_id) && s.thread_requests.len() >= 4096
            {
                return Err(Error::Limit);
            }
            s.thread_requests
                .insert(input.request_id, (home, profile.clone(), request.clone()));
            Ok(())
        })?;
        let result = self
            .apply_home_thread_command(home, &profile, &request)
            .await?
            .ok_or(Error::Unavailable)?;
        self.execution_changed();
        Ok(api::Outcome {
            request_id: input.request_id,
            home,
            thread: result.thread_id,
            revision: result.version,
        })
    }
    pub(super) async fn validate_thread_workspace(&self, session: &str) -> Result<()> {
        let saved = self.saved(session)?;
        let origin = self.read(|s| {
            Ok(saved
                .view
                .conversation_id
                .as_ref()
                .and_then(|id| s.conversations.get(id))
                .map(|c| c.origin.clone()))
        })?;
        let Some(origin) = origin else {
            return Ok(());
        };
        let Some(thread) = origin.thread else {
            return Ok(());
        };
        let selected = self
            .home_thread_summary(saved.view.repository, &saved.profile, thread)
            .await?;
        if let Some(codebase) = selected.value.codebase
            && saved
                .code_workspaces
                .keys()
                .any(|repository| !codebase.contains(repository))
        {
            return Err(Error::Stale);
        }
        Ok(())
    }
    pub(super) fn thread_has_pending_work(
        &self,
        home: RepositoryId,
        thread: ThreadId,
    ) -> Result<bool> {
        self.read(|s| {
            let ids: std::collections::BTreeSet<_> = s
                .conversations
                .values()
                .filter(|c| c.home == home && c.origin.thread == Some(thread))
                .flat_map(|c| c.participants.iter().cloned())
                .collect();
            Ok(s.sessions.values().any(|session| {
                ids.contains(&session.view.id)
                    && matches!(
                        session.view.state.as_str(),
                        "running" | "preparing" | "opening"
                    )
            }) || s.intents.values().any(|intent| {
                intent
                    .outcome
                    .session_id
                    .as_ref()
                    .is_some_and(|id| ids.contains(id))
                    && (matches!(
                        intent.outcome.state.as_str(),
                        "queued" | "dispatching" | "started" | "outcome_unknown" | "interrupted"
                    ) || intent.outcome.state.starts_with("preparing_"))
            }))
        })
    }
}

fn removes_repository(
    previous: Option<&[RepositoryId]>,
    home: RepositoryId,
    next: &[RepositoryId],
) -> bool {
    // Legacy user Threads implicitly select their home repository, matching
    // conversation admission. Explicit empty Codebases select no code.
    previous
        .unwrap_or(std::slice::from_ref(&home))
        .iter()
        .any(|repository| !next.contains(repository))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_codebase_removal_uses_the_same_home_fallback_as_send() {
        let home = RepositoryId::new([1; 16]);
        let other = RepositoryId::new([2; 16]);
        assert!(removes_repository(None, home, &[]));
        assert!(removes_repository(None, home, &[other]));
        assert!(!removes_repository(None, home, &[home, other]));
        assert!(!removes_repository(Some(&[]), home, &[]));
        assert!(removes_repository(Some(&[home, other]), home, &[other]));
    }
}
