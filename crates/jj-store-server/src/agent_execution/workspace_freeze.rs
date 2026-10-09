//! Capture all current selections before any review publication; retain every frozen step.
use super::workspace_sources::{CapturedFile, verify_file_bytes};
use super::*;
use jj_collab::ConversationSubmissionV1;
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct WorkspaceSnapshot {
    pub completed: Option<Workspace>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct WorkspaceSubmission {
    pub original: ConversationSubmissionV1,
    pub snapshots: BTreeMap<String, WorkspaceSnapshot>,
    pub captures: BTreeMap<RequestId, CapturedFile>,
    pub resolved: Option<ConversationSubmissionV1>,
    #[serde(default)]
    pub refused: bool,
}
impl Supervisor {
    pub(super) async fn resolve_workspace_comments(
        &self,
        request: RequestId,
        original: &ConversationSubmissionV1,
    ) -> Result<ConversationSubmissionV1> {
        original.validate().map_err(|_| Error::Invalid)?;
        let prior = self.read(|s| Ok(s.workspace_submissions.get(&request).cloned()))?;
        if let Some(prior) = prior {
            if prior.original != *original {
                return Err(Error::Conflict);
            }
            if prior.refused {
                return Err(Error::Stale);
            }
            if let Some(resolved) = prior.resolved {
                return Ok(resolved);
            }
        } else {
            self.update(|s| {
                if s.workspace_submissions.len() >= 4096 {
                    return Err(Error::Limit);
                }
                s.workspace_submissions.insert(
                    request,
                    WorkspaceSubmission {
                        original: original.clone(),
                        snapshots: BTreeMap::new(),
                        captures: BTreeMap::new(),
                        resolved: None,
                        refused: false,
                    },
                );
                Ok(())
            })?;
        }
        // Already-frozen drafts have stable independent receipts. Reusing them in a
        // recovered parent must not capture a newer version or create another review.
        let mut pending = vec![];
        for draft in &original.workspace_comments {
            if let Some(prior) = self.read(|s| Ok(s.file_publications.get(&draft.id).cloned()))? {
                if prior.draft != *draft {
                    return Err(Error::Conflict);
                }
                self.authorized(draft.source.repository, &prior.profile, true)
                    .await?;
                continue;
            }
            let saved = self.saved(&draft.source.session.to_string())?;
            let (workspace, profile) =
                workspace_inspection::binding(&saved, draft.source.repository)?;
            self.authorized(saved.view.repository, &saved.profile, false)
                .await?;
            self.authorized(draft.source.repository, profile, true)
                .await?;
            if workspace.name != draft.source.workspace {
                return Err(Error::Stale);
            }
            if draft.source.current_fingerprint.is_some()
                && (saved.view.state == "running"
                    || self
                        .0
                        .workspace_activity
                        .lock()
                        .map_err(|_| Error::Unavailable)?
                        .contains_key(&saved.view.id))
            {
                return Err(Error::Busy);
            }
            pending.push((draft.clone(), workspace.clone(), profile.to_owned()));
        }
        // Validate every current file before the first physical snapshot. Each read
        // is descriptor-relative and must agree with the exact UI observation.
        for (draft, workspace, _) in &pending {
            let key = format!("{}/{}", draft.source.session, draft.source.repository);
            if self.read(|s| {
                Ok(s.workspace_submissions[&request]
                    .snapshots
                    .contains_key(&key))
            })? {
                continue;
            }
            if let Some(fingerprint) = &draft.source.current_fingerprint {
                let root = workspace.path.clone();
                let repository = draft.source.repository;
                let path = draft.source.path.clone();
                let file = tokio::task::spawn_blocking(move || {
                    jj_store_service::working_copy::read(&root, repository, 0, &path)
                })
                .await
                .map_err(|_| Error::Unavailable)?
                .map_err(|_| Error::Stale)?;
                if file.selection.fingerprint != *fingerprint {
                    return Err(Error::Stale);
                }
                verify_file_bytes(&draft.source, file.contents.as_bytes())?;
            }
        }
        for (draft, workspace, profile) in &pending {
            if self.read(|s| {
                Ok(s.workspace_submissions[&request]
                    .captures
                    .contains_key(&draft.id))
            })? {
                continue;
            }
            let mut selected = draft.source.clone();
            if selected.current_fingerprint.is_some() {
                let key = format!("{}/{}", selected.session, selected.repository);
                let old = self.read(|s| {
                    Ok(s.workspace_submissions[&request]
                        .snapshots
                        .get(&key)
                        .cloned())
                })?;
                let snapshot = if let Some(old) = old {
                    // A missing completion is an unknown physical outcome. Never resnapshot it.
                    old.completed.ok_or(Error::Unavailable)?
                } else {
                    self.update(|s| {
                        s.workspace_submissions
                            .get_mut(&request)
                            .ok_or(Error::Invalid)?
                            .snapshots
                            .insert(key.clone(), WorkspaceSnapshot { completed: None });
                        Ok(())
                    })?;
                    self.authorized(selected.repository, profile, true).await?;
                    let snapshot = self.0.resources.driver.snapshot(workspace).await?;
                    self.update(|s| {
                        s.workspace_submissions
                            .get_mut(&request)
                            .ok_or(Error::Invalid)?
                            .snapshots
                            .get_mut(&key)
                            .ok_or(Error::Invalid)?
                            .completed = Some(snapshot.clone());
                        Ok(())
                    })?;
                    snapshot
                };
                selected.operation = snapshot.operation;
                selected.commit = snapshot.commit.clone();
                selected.current_fingerprint = None;
                if let Some(diff) = &mut selected.diff {
                    diff.after_operation = snapshot.operation;
                    diff.after_commit = snapshot.commit;
                }
            }
            let captured = if let Some(previous) = &draft.previous {
                self.retained_file_anchor(&selected, previous).await?
            } else {
                self.committed_file_anchor(workspace, profile, &selected)
                    .await?
            };
            self.update(|s| {
                s.workspace_submissions
                    .get_mut(&request)
                    .ok_or(Error::Invalid)?
                    .captures
                    .insert(draft.id, captured);
                Ok(())
            })?;
        }
        let mut resolved = original.clone();
        resolved.workspace_comments.clear();
        let mut groups: Vec<(CapturedFile, jj_collab::ReactionTargetV1)> = vec![];
        for draft in &original.workspace_comments {
            let old = self.read(|s| Ok(s.file_publications.get(&draft.id).cloned()))?;
            let (captured, profile) = if let Some(old) = old {
                (old.captured, old.profile)
            } else {
                let (_, _, profile) = pending
                    .iter()
                    .find(|(d, _, _)| d.id == draft.id)
                    .ok_or(Error::Invalid)?;
                let captured = self.read(|s| {
                    s.workspace_submissions[&request]
                        .captures
                        .get(&draft.id)
                        .cloned()
                        .ok_or(Error::Invalid)
                })?;
                (captured, profile.clone())
            };
            let previous = draft.previous.as_ref().or_else(|| {
                groups
                    .iter()
                    .find(|(a, _)| {
                        a.anchor.target == captured.anchor.target
                            && a.anchor.repository_id == captured.anchor.repository_id
                    })
                    .map(|(_, r)| r)
            });
            let target = self
                .publish_captured_file_draft(draft, &profile, captured.clone(), previous)
                .await?;
            groups.push((captured, target.clone()));
            if !resolved.references.contains(&target) {
                resolved.references.push(target);
            }
        }
        resolved.validate().map_err(|_| Error::Invalid)?;
        self.update(|s| {
            s.workspace_submissions
                .get_mut(&request)
                .ok_or(Error::Invalid)?
                .resolved = Some(resolved.clone());
            Ok(())
        })?;
        Ok(resolved)
    }
}
