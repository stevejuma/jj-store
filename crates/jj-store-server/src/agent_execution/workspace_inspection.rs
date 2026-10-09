//! Inspection selects a retained participant binding, never a browser filesystem path.
mod comparison;
mod content;
mod source;
pub(super) mod watch;
use super::*;
use jj_store_service::agent_execution::workspace_inspection as dto;
use sha2::{Digest as _, Sha256};
use std::collections::BTreeSet;
impl Supervisor {
    pub(super) async fn workspace_inspection(
        &self,
        actor: PrincipalId,
        request: dto::Request,
    ) -> Result<dto::Inspection> {
        let _permit = self
            .0
            .inspection_slots
            .try_acquire()
            .map_err(|_| Error::Busy)?;
        self.actor(actor)?;
        request.validate()?;
        let saved = self.saved(&request.session)?;
        self.authorized(saved.view.repository, &saved.profile, false)
            .await?;
        let (proof, profile) = binding(&saved, request.repository)?;
        self.authorized(request.repository, profile, false).await?;
        let selected = self.0.resources.driver.inspect(proof.path.clone()).await?;
        if selected.path != proof.path
            || selected.metadata != proof.metadata
            || selected.name != proof.name
        {
            return Err(Error::Stale);
        }
        self.watch_inspected_workspace(&request.session, request.repository, &selected)?;
        let source = self
            .workspace_source(request.repository, profile, selected.operation)
            .await?;
        let target = jj_store_jj::local_stacks::configured_target(&proof.source)
            .map_err(|_| Error::Unavailable)?;
        let facts = source.revision(&selected.commit, target.as_deref()).await?;
        let comparison = match &request.query {
            dto::InspectionView::Files => None,
            dto::InspectionView::Changes { range, base } => Some(
                self.workspace_comparison(
                    &saved,
                    request.repository,
                    &selected,
                    &facts,
                    &source,
                    *range,
                    base.as_deref(),
                )
                .await?,
            ),
        };
        let mut after = selected.clone();
        if let Some(c) = &comparison {
            after.operation = c.after_operation;
            after.commit = c.after_commit.clone();
        }
        let current = comparison.as_ref().is_none_or(|c| c.current_files);
        let historical_source = if after.operation != selected.operation {
            Some(
                self.workspace_source(request.repository, profile, after.operation)
                    .await?,
            )
        } else {
            None
        };
        let after_source = historical_source.as_ref().unwrap_or(&source);
        let before_source = if let Some(c) = &comparison
            && c.unavailable.is_none()
        {
            Some(
                self.workspace_source(
                    request.repository,
                    profile,
                    c.before_operation.ok_or(Error::Unavailable)?,
                )
                .await?,
            )
        } else {
            None
        };
        let tree = after_source.files(&after.commit).await?;
        let tracked: BTreeSet<_> = tree.iter().map(|e| e.path.clone()).collect();
        let observed_entries = if current {
            let root = selected.path.clone();
            let tracked = tracked.clone();
            let ignores = self.0.resources.driver.ignore_inputs(&selected)?;
            tokio::task::spawn_blocking(move || dto::files::list(&root, &tracked, ignores))
                .await
                .map_err(|_| Error::Unavailable)??
        } else {
            tree.iter()
                .map(|entry| dto::FileEntry {
                    path: entry.path.clone(),
                    kind: entry.kind.clone(),
                    bytes: 0,
                    fingerprint: String::new(),
                    status: None,
                })
                .collect()
        };
        let mut entries = observed_entries.clone();
        let mut incomplete = None;
        if let Some(c) = &comparison {
            entries = if c.unavailable.is_none() {
                self.workspace_changed_paths(
                    &after,
                    request.repository,
                    c,
                    entries,
                    &after_source,
                    before_source.as_ref().ok_or(Error::Unavailable)?,
                )
                .await
                .map(|(paths, reason)| {
                    incomplete = reason;
                    paths
                })?
            } else {
                vec![]
            };
        }
        let mut hash = Sha256::new_with_prefix(b"jj-store/workspace-inspection/v1\0");
        hash.update(request.session.as_bytes());
        hash.update(request.repository.as_bytes());
        hash.update(selected.operation.as_bytes());
        hash.update(selected.commit.as_bytes());
        hash.update(serde_json::to_vec(&comparison).map_err(|_| Error::Invalid)?);
        hash.update(serde_json::to_vec(&entries).map_err(|_| Error::Invalid)?);
        let selection = hex::encode(hash.finalize());
        let offset = match request.cursor.as_deref() {
            None => 0,
            Some(value) => {
                let (fingerprint, offset) = value.split_once(':').ok_or(Error::Invalid)?;
                if fingerprint != selection {
                    return Err(Error::Stale);
                }
                offset.parse::<usize>().map_err(|_| Error::Invalid)?
            }
        };
        if offset > entries.len() {
            return Err(Error::Invalid);
        }
        let file = if let Some(path) = &request.path {
            let entry = entries
                .iter()
                .find(|e| &e.path == path)
                .ok_or(Error::Stale)?;
            Some(
                self.workspace_content(
                    &after,
                    request.repository,
                    path,
                    comparison.as_ref(),
                    &after_source,
                    before_source.as_ref(),
                    entry.status.as_deref() != Some("deleted"),
                    comparison.is_some() && entry.status.as_deref() != Some("added"),
                )
                .await?,
            )
        } else {
            None
        };
        let end = (offset + 50).min(entries.len());
        let next = (end < entries.len()).then(|| format!("{selection}:{end}"));
        let files = entries.drain(offset..end).collect();
        if current {
            let root = selected.path.clone();
            let ignores = self.0.resources.driver.ignore_inputs(&selected)?;
            let reread =
                tokio::task::spawn_blocking(move || dto::files::list(&root, &tracked, ignores))
                    .await
                    .map_err(|_| Error::Unavailable)??;
            if reread != observed_entries {
                return Err(Error::Stale);
            }
        }
        self.authorized(saved.view.repository, &saved.profile, false)
            .await?;
        self.authorized(request.repository, profile, false).await?;
        self.0.resources.driver.verify(&selected).await?;
        Ok(dto::Inspection {
            version: 1,
            session: request.session,
            repository: request.repository,
            workspace: selected.name,
            location: selected.path.to_string_lossy().into(),
            operation: selected.operation,
            commit: selected.commit,
            selection,
            references: facts.references,
            comparison,
            incomplete,
            files,
            next,
            file,
        })
    }
}
pub(super) fn binding(
    saved: &SavedSession,
    repository: RepositoryId,
) -> Result<(&Workspace, &str)> {
    if let Some(proof) = saved.code_workspaces.get(&repository) {
        let binding = saved
            .view
            .repository_bindings
            .iter()
            .find(|b| b.repository == repository)
            .ok_or(Error::Stale)?;
        return Ok((proof, &binding.profile));
    }
    if saved.view.conversation_id.is_none()
        && saved.view.repository == repository
        && saved.workspace_ready
    {
        return Ok((&saved.workspace, &saved.profile));
    }
    Err(Error::Denied)
}
