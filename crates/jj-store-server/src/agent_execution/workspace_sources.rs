//! Convert a verified committed selection to the owning provider's immutable anchor.
use super::*;
use super::{Error, Result};
use jj_collab::WorkspaceFileSourceV1;
use jj_store_model::portable::*;
use jj_store_service::{local_repository::LocalCodeEvidenceV1, review_model::*};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CapturedFile {
    pub anchor: ArtifactAnchorV1,
    pub evidence: Vec<LocalCodeEvidenceV1>,
}
impl Supervisor {
    pub(super) async fn committed_file_anchor(
        &self,
        workspace: &Workspace,
        profile: &str,
        selected: &WorkspaceFileSourceV1,
    ) -> Result<CapturedFile> {
        self.authorized(selected.repository, profile, true).await?;
        let mut evidence = Vec::new();
        let (before_path, after_path) = if let Some(diff) = &selected.diff {
            let before = self
                .workspace_source(selected.repository, profile, diff.before_operation)
                .await?;
            let after = self
                .workspace_source(selected.repository, profile, diff.after_operation)
                .await?;
            (
                before
                    .files(&diff.before_commit)
                    .await?
                    .iter()
                    .any(|e| e.path == selected.path)
                    .then(|| selected.path.clone()),
                after
                    .files(&diff.after_commit)
                    .await?
                    .iter()
                    .any(|e| e.path == selected.path)
                    .then(|| selected.path.clone()),
            )
        } else {
            (None, None)
        };
        let target = if profile == "local_jj_git" {
            let local = self
                .0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(selected.repository)
                .map_err(|_| Error::Denied)?;
            let mut revisions = BTreeMap::new();
            let mut needed = vec![(selected.operation, selected.commit.clone())];
            if let Some(diff) = &selected.diff {
                needed.extend([
                    (diff.before_operation, diff.before_commit.clone()),
                    (diff.after_operation, diff.after_commit.clone()),
                ]);
            }
            for (operation, commit) in needed {
                if revisions.contains_key(&(operation, commit.clone())) {
                    continue;
                }
                let capsule = jj_store_jj::agent_workspaces::inspection::evidence(
                    &workspace.metadata,
                    selected.repository,
                    operation,
                )
                .await
                .map_err(|_| Error::Unavailable)?;
                let commit_id = commit.parse().map_err(|_| Error::Invalid)?;
                let value = capsule.commits.get(&commit_id).ok_or(Error::Stale)?;
                let reference = CodeEvidenceRefV1 {
                    repository: selected.repository,
                    operation,
                    profile: CodeProfileV1::GitSha1Jj045,
                    capsule: capsule.id().map_err(|_| Error::Invalid)?,
                };
                revisions.insert(
                    (operation, commit),
                    PortableRevisionV2 {
                        evidence: reference.clone(),
                        commit: CodeCommitIdV1::GitSha1Jj045(commit_id),
                        change: Some(value.change_id),
                    },
                );
                if !evidence
                    .iter()
                    .any(|e: &LocalCodeEvidenceV1| e.reference == reference)
                {
                    evidence.push(LocalCodeEvidenceV1 {
                        reference,
                        bytes: capsule.encode().map_err(|_| Error::Limit)?,
                    });
                }
            }
            let revision = revisions[&(selected.operation, selected.commit.clone())].clone();
            let capsule = evidence
                .iter()
                .find(|e| e.reference == revision.evidence)
                .ok_or(Error::Invalid)?
                .validate()
                .map_err(|_| Error::Invalid)?;
            let (id, bytes) = local
                .metadata_store()
                .read_capsule_file(
                    &capsule,
                    selected.commit.parse().map_err(|_| Error::Invalid)?,
                    &selected.path,
                )
                .map_err(|_| Error::Unavailable)?;
            verify_file_bytes(selected, &bytes)?;
            let artifact = PortableCodeArtifactV2 {
                revision,
                path: selected.path.clone(),
                object: CodeObjectIdV1::GitSha1Jj045 {
                    kind: GitObjectKindV1::Blob,
                    id,
                },
                value_kind: CodeValueKind::File,
            };
            if let Some(text) = &selected.selection {
                ArtifactTargetV1::PortableCodeText(Box::new(PortableCodeTextV2 {
                    artifact,
                    text: text.clone(),
                    diff_context: selected.diff.as_ref().map(|diff| PortableDiffContextV2 {
                        before: revisions[&(diff.before_operation, diff.before_commit.clone())]
                            .clone(),
                        after: revisions[&(diff.after_operation, diff.after_commit.clone())]
                            .clone(),
                        before_path: before_path.clone(),
                        after_path: after_path.clone(),
                        side: diff.side,
                    }),
                }))
            } else {
                // Whole-file identity remains exact even for empty text.
                ArtifactTargetV1::PortableCodeArtifact(Box::new(artifact))
            }
        } else {
            let source = self
                .workspace_source(selected.repository, profile, selected.operation)
                .await?;
            let entries = source.files(&selected.commit).await?;
            let id = entries
                .iter()
                .find(|e| e.path == selected.path)
                .and_then(|e| e.file.as_ref())
                .ok_or(Error::Unavailable)?
                .parse()
                .map_err(|_| Error::Invalid)?;
            let text = source.file(&selected.commit, &selected.path).await?;
            verify_file_bytes(selected, text.as_bytes())?;
            let commit_id = selected.commit.parse().map_err(|_| Error::Invalid)?;
            if let Some(text) = &selected.selection {
                let diff_context = selected
                    .diff
                    .as_ref()
                    .map(|diff| {
                        Ok::<_, Error>(DiffContextV1 {
                            before_operation_id: diff.before_operation,
                            before_commit_id: diff
                                .before_commit
                                .parse()
                                .map_err(|_| Error::Invalid)?,
                            before_path: before_path.clone(),
                            after_operation_id: diff.after_operation,
                            after_commit_id: diff
                                .after_commit
                                .parse()
                                .map_err(|_| Error::Invalid)?,
                            after_path: after_path.clone(),
                            side: diff.side,
                            comparison: ComparisonKind::ExplicitPair,
                        })
                    })
                    .transpose()?;
                ArtifactTargetV1::CodeText(Box::new(CodeTextAnchorV1 {
                    operation_id: selected.operation,
                    commit_id,
                    change_id: None,
                    path: selected.path.clone(),
                    file_id: id,
                    value_kind: CodeValueKind::File,
                    text: text.clone(),
                    diff_context,
                }))
            } else {
                ArtifactTargetV1::CodeArtifact(Box::new(CodeArtifactAnchorV1 {
                    operation_id: selected.operation,
                    commit_id,
                    change_id: None,
                    path: selected.path.clone(),
                    object: jj_store_model::ObjectKey::File(id),
                    value_kind: CodeValueKind::File,
                }))
            }
        };
        let anchor = ArtifactAnchorV1 {
            schema_version: 1,
            repository_id: selected.repository,
            target,
            created_context: CreatedContextV1 {
                publication_position: None,
                code_state_version: None,
                agent_root: None,
            },
            sensitivity: SensitivityV1::Repository,
        };
        anchor.validate().map_err(|_| Error::Invalid)?;
        self.authorized(selected.repository, profile, true).await?;
        let required = anchor.code_evidence().map_err(|_| Error::Invalid)?;
        evidence.retain(|e| required.contains(&e.reference));
        Ok(CapturedFile { anchor, evidence })
    }
}
pub(super) fn verify_file_bytes(source: &WorkspaceFileSourceV1, bytes: &[u8]) -> Result<()> {
    if bytes.len() > 256 * 1024
        || bytes.contains(&0)
        || ReviewDigest::sha256(bytes) != source.content_sha256
    {
        return Err(Error::Stale);
    }
    std::str::from_utf8(bytes).map_err(|_| Error::Invalid)?;
    if let Some(selection) = &source.selection {
        selection.validate_bytes(bytes).map_err(|_| Error::Stale)?;
    }
    Ok(())
}

impl Supervisor {
    pub(super) async fn retained_file_anchor(
        &self,
        source: &WorkspaceFileSourceV1,
        target: &jj_collab::ReactionTargetV1,
    ) -> Result<CapturedFile> {
        use jj_collab::QualifiedReviewRead;
        if source.current_fingerprint.is_some() {
            return Err(Error::Stale);
        }
        let retained = self.comment(target).await.map_err(|_| Error::Unavailable)?;
        verify_file_bytes(source, &retained.file)?;
        let (operation, commit, path, selection) = match &retained.anchor.target {
            ArtifactTargetV1::CodeText(v) => (
                v.operation_id,
                v.commit_id.to_string(),
                &v.path,
                Some(&v.text),
            ),
            ArtifactTargetV1::CodeArtifact(v) => {
                (v.operation_id, v.commit_id.to_string(), &v.path, None)
            }
            ArtifactTargetV1::PortableCodeText(v) => (
                v.artifact.revision.evidence.operation,
                portable_commit(&v.artifact.revision)?,
                &v.artifact.path,
                Some(&v.text),
            ),
            ArtifactTargetV1::PortableCodeArtifact(v) => (
                v.revision.evidence.operation,
                portable_commit(&v.revision)?,
                &v.path,
                None,
            ),
            _ => return Err(Error::Invalid),
        };
        if source.repository != retained.anchor.repository_id
            || operation != source.operation
            || commit != source.commit
            || *path != source.path
            || selection != source.selection.as_ref()
        {
            return Err(Error::Stale);
        }
        // The existing review's exact anchor, including diff context, is authoritative.
        Ok(CapturedFile {
            anchor: retained.anchor,
            evidence: vec![],
        })
    }
}
fn portable_commit(revision: &PortableRevisionV2) -> Result<String> {
    match revision.commit {
        CodeCommitIdV1::GitSha1Jj045(id) => Ok(id.to_string()),
        _ => Err(Error::Invalid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn captured_bytes_require_exact_unicode_boundaries_and_full_source_hash() {
        let bytes = "café 🦀\n".as_bytes();
        let mut source = WorkspaceFileSourceV1 {
            session: RequestId::new([1; 16]),
            repository: RepositoryId::new([2; 16]),
            workspace: "agent".into(),
            path: "source.rs".into(),
            operation: OperationId::new([3; 64]),
            commit: "ab".repeat(20),
            content_sha256: ReviewDigest::sha256(bytes),
            current_fingerprint: None,
            selection: Some(TextSelectionV1 {
                encoding: TextEncoding::Utf8,
                start_byte: ByteOffset::new(6),
                end_byte: ByteOffset::new(10),
                selected_sha256: ReviewDigest::sha256("🦀".as_bytes()),
            }),
            diff: None,
        };
        verify_file_bytes(&source, bytes).unwrap();
        assert!(verify_file_bytes(&source, "café 🦀 later\n".as_bytes()).is_err());
        source.selection.as_mut().unwrap().start_byte = ByteOffset::new(7);
        assert!(verify_file_bytes(&source, bytes).is_err());
    }
}
