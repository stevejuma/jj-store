use crate::{
    AgentContentOwnerV1, AgentTextAnchorV1, ArtifactAnchorV1, ArtifactTargetV1,
    CodeArtifactAnchorV1, CodeTextAnchorV1, CodeValueKind, DiffSide, Error, Result, valid_label,
    valid_repo_path,
};
use jj_agent::{AgentObjectKind, AgentObjectRef};
use jj_store_model::ObjectKey;

impl ArtifactAnchorV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 {
            return Err(Error::Invalid("anchor schema"));
        }
        if self
            .code_evidence_unchecked()
            .iter()
            .any(|e| e.repository != self.repository_id)
        {
            return Err(Error::Invalid("portable anchor repository"));
        }
        match &self.target {
            ArtifactTargetV1::PortableCodeText(value) => value.validate(),
            ArtifactTargetV1::PortableCodeArtifact(value) => value.validate(),
            ArtifactTargetV1::CodeText(value) => validate_code_text(value),
            ArtifactTargetV1::CodeArtifact(value) => validate_code_artifact(value),
            ArtifactTargetV1::AgentText(value) => validate_agent_text(value),
            ArtifactTargetV1::AgentArtifact(value) => {
                validate_agent_owner(value.session_snapshot, &value.owner)
            }
            ArtifactTargetV1::AgentSession {
                session_snapshot, ..
            } => {
                if session_snapshot.kind == AgentObjectKind::SessionSnapshot {
                    Ok(())
                } else {
                    Err(Error::Invalid("agent session snapshot"))
                }
            }
            ArtifactTargetV1::Revision { .. }
            | ArtifactTargetV1::Operation { .. }
            | ArtifactTargetV1::Repository => Ok(()),
        }
    }

    pub fn validate_text(&self, bytes: &[u8]) -> Result<()> {
        self.validate()?;
        match &self.target {
            ArtifactTargetV1::CodeText(value) => value.text.validate_bytes(bytes),
            ArtifactTargetV1::PortableCodeText(value) => value.text.validate_bytes(bytes),
            ArtifactTargetV1::AgentText(value) => value.text.validate_bytes(bytes),
            _ => Err(Error::Invalid("anchor is not text")),
        }
    }
}

fn validate_code_text(value: &CodeTextAnchorV1) -> Result<()> {
    if value.value_kind != CodeValueKind::File || !valid_repo_path(&value.path) {
        return Err(Error::Invalid("code text artifact"));
    }
    value.text.validate()?;
    if let Some(diff) = &value.diff_context {
        let selected = match diff.side {
            DiffSide::Before => (
                &diff.before_path,
                diff.before_operation_id,
                diff.before_commit_id,
            ),
            DiffSide::After => (
                &diff.after_path,
                diff.after_operation_id,
                diff.after_commit_id,
            ),
        };
        if selected.0.as_deref() != Some(&value.path)
            || selected.1 != value.operation_id
            || selected.2 != value.commit_id
            || diff
                .before_path
                .as_deref()
                .is_some_and(|path| !valid_repo_path(path))
            || diff
                .after_path
                .as_deref()
                .is_some_and(|path| !valid_repo_path(path))
        {
            return Err(Error::Invalid("diff text identity"));
        }
    }
    Ok(())
}

fn validate_code_artifact(value: &CodeArtifactAnchorV1) -> Result<()> {
    let kind_matches = matches!(
        (value.value_kind, value.object),
        (
            CodeValueKind::File
                | CodeValueKind::BinaryFile
                | CodeValueKind::EmptyFile
                | CodeValueKind::ConflictTerm
                | CodeValueKind::ConflictArtifact,
            ObjectKey::File(_)
        ) | (CodeValueKind::Symlink, ObjectKey::Symlink(_))
    );
    if valid_repo_path(&value.path) && kind_matches {
        Ok(())
    } else {
        Err(Error::Invalid("code artifact identity"))
    }
}

fn validate_agent_text(value: &AgentTextAnchorV1) -> Result<()> {
    validate_agent_owner(value.session_snapshot, &value.owner)?;
    if value.block_version.kind != AgentObjectKind::Content
        || value.content.kind != AgentObjectKind::Content
        || !valid_label(&value.representation_version, 128)
        || !valid_label(&value.sanitizer_version, 128)
        || !valid_label(&value.normalizer_version, 128)
    {
        return Err(Error::Invalid("agent text identity"));
    }
    value.text.validate()
}

fn validate_agent_owner(session: AgentObjectRef, owner: &AgentContentOwnerV1) -> Result<()> {
    let valid = session.kind == AgentObjectKind::SessionSnapshot
        && match owner {
            AgentContentOwnerV1::Tool { snapshot, .. } => {
                snapshot.kind == AgentObjectKind::ToolSnapshot
            }
            AgentContentOwnerV1::Message { snapshot, .. }
            | AgentContentOwnerV1::Plan { snapshot, .. } => {
                snapshot.kind == AgentObjectKind::MessageSnapshot
            }
        };
    if valid {
        Ok(())
    } else {
        Err(Error::Invalid("agent owner snapshot"))
    }
}
