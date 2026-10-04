//! Additive portable anchor schema; the native V1 schema is unchanged.
use crate::{
    AnchorId, ArtifactId, CodeValueKind, CreatedContextV1, DiffSide, Error, PortableAgentSessionV2,
    PortableAgentToolTextV2, PortableAgentToolV2, Result, SensitivityV1, TextSelectionV1,
    valid_repo_path,
};
use jj_agent::AgentObjectRef;
use jj_store_model::{ChangeId, ObjectKey, RepositoryId, portable::*};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PortableRevisionV2 {
    pub evidence: CodeEvidenceRefV1,
    pub commit: CodeCommitIdV1,
    pub change: Option<ChangeId>,
}
impl PortableRevisionV2 {
    pub fn validate(&self) -> Result<()> {
        if self.evidence.profile != self.commit.profile() {
            return Err(Error::Invalid("source commit profile"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PortableDiffContextV2 {
    pub before: PortableRevisionV2,
    pub before_path: Option<String>,
    pub after: PortableRevisionV2,
    pub after_path: Option<String>,
    pub side: DiffSide,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PortableCodeArtifactV2 {
    pub revision: PortableRevisionV2,
    pub path: String,
    pub object: CodeObjectIdV1,
    pub value_kind: CodeValueKind,
}
impl PortableCodeArtifactV2 {
    pub fn validate(&self) -> Result<()> {
        self.revision.validate()?;
        let kind = match self.object {
            CodeObjectIdV1::NativeV1 { object } => matches!(
                (self.value_kind, object),
                (
                    CodeValueKind::File
                        | CodeValueKind::BinaryFile
                        | CodeValueKind::EmptyFile
                        | CodeValueKind::ConflictTerm
                        | CodeValueKind::ConflictArtifact,
                    ObjectKey::File(_)
                ) | (CodeValueKind::Symlink, ObjectKey::Symlink(_))
            ),
            CodeObjectIdV1::GitSha1Jj045 {
                kind: GitObjectKindV1::Blob,
                id,
            } => {
                id.as_bytes() != &[0; 20]
                    && matches!(
                        self.value_kind,
                        CodeValueKind::File
                            | CodeValueKind::BinaryFile
                            | CodeValueKind::EmptyFile
                            | CodeValueKind::Symlink
                            | CodeValueKind::ConflictTerm
                            | CodeValueKind::ConflictArtifact
                    )
            }
            _ => false,
        };
        if !valid_repo_path(&self.path)
            || !kind
            || self.object.profile() != self.revision.evidence.profile
        {
            return Err(Error::Invalid("portable artifact identity"));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PortableCodeTextV2 {
    pub artifact: PortableCodeArtifactV2,
    pub text: TextSelectionV1,
    pub diff_context: Option<PortableDiffContextV2>,
}
impl PortableCodeTextV2 {
    pub fn validate(&self) -> Result<()> {
        self.artifact.validate()?;
        self.text.validate()?;
        if self.artifact.value_kind != CodeValueKind::File {
            return Err(Error::Invalid("portable text value"));
        }
        if let Some(diff) = &self.diff_context {
            diff.before.validate()?;
            diff.after.validate()?;
            let (revision, path) = match diff.side {
                DiffSide::Before => (&diff.before, &diff.before_path),
                DiffSide::After => (&diff.after, &diff.after_path),
            };
            if revision != &self.artifact.revision
                || path.as_deref() != Some(&self.artifact.path)
                || diff.before.evidence.repository != diff.after.evidence.repository
                || diff
                    .before_path
                    .as_deref()
                    .is_some_and(|p| !valid_repo_path(p))
                || diff
                    .after_path
                    .as_deref()
                    .is_some_and(|p| !valid_repo_path(p))
            {
                return Err(Error::Invalid("portable diff identity"));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "target",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ArtifactTargetV2 {
    CodeText(Box<PortableCodeTextV2>),
    CodeArtifact(Box<PortableCodeArtifactV2>),
    Revision(PortableRevisionV2),
    Operation(CodeEvidenceRefV1),
    AgentSession(PortableAgentSessionV2),
    AgentToolArtifact(PortableAgentToolV2),
    AgentToolText(Box<PortableAgentToolTextV2>),
    Repository,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArtifactAnchorV2 {
    pub schema_version: u32,
    pub repository_id: RepositoryId,
    pub target: ArtifactTargetV2,
    pub created_context: CreatedContextV1,
    pub sensitivity: SensitivityV1,
}
impl ArtifactAnchorV2 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 2 {
            return Err(Error::Invalid("portable anchor schema"));
        }
        if self
            .code_evidence_unchecked()
            .iter()
            .any(|evidence| evidence.repository != self.repository_id)
        {
            return Err(Error::Invalid("portable anchor evidence repository"));
        }
        match &self.target {
            ArtifactTargetV2::CodeText(value) => value.validate(),
            ArtifactTargetV2::CodeArtifact(value) => value.validate(),
            ArtifactTargetV2::Revision(value) => value.validate(),
            ArtifactTargetV2::AgentSession(value) => value.validate(),
            ArtifactTargetV2::AgentToolArtifact(value) => value.validate(),
            ArtifactTargetV2::AgentToolText(value) => value.validate(),
            ArtifactTargetV2::Operation(_) | ArtifactTargetV2::Repository => Ok(()),
        }
    }
    pub fn validate_text(&self, bytes: &[u8]) -> Result<()> {
        self.validate()?;
        match &self.target {
            ArtifactTargetV2::CodeText(value) => value.text.validate_bytes(bytes),
            ArtifactTargetV2::AgentToolText(value) => value.text.validate_bytes(bytes),
            _ => Err(Error::Invalid("anchor is not text")),
        }
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        crate::codec::encode(self, 1024 * 1024)
    }
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let value: Self = crate::codec::decode(bytes, 1024 * 1024)?;
        value.validate()?;
        Ok(value)
    }
    pub fn anchor_id(&self) -> Result<AnchorId> {
        Ok(AnchorId::new(Sha256::digest(self.encode()?).into()))
    }
    pub fn artifact_id(&self) -> Result<ArtifactId> {
        self.validate()?;
        let bytes = crate::codec::encode(&self.target, 1024 * 1024)?;
        let mut hash = Sha256::new_with_prefix(b"jj-store/review-artifact/v2\0");
        hash.update((bytes.len() as u64).to_be_bytes());
        hash.update(bytes);
        Ok(ArtifactId::new(hash.finalize().into()))
    }
    pub fn code_evidence(&self) -> Result<BTreeSet<CodeEvidenceRefV1>> {
        self.validate()?;
        Ok(self.code_evidence_unchecked())
    }
    fn code_evidence_unchecked(&self) -> BTreeSet<CodeEvidenceRefV1> {
        match &self.target {
            ArtifactTargetV2::CodeText(value) => {
                let mut values = BTreeSet::from([value.artifact.revision.evidence.clone()]);
                if let Some(diff) = &value.diff_context {
                    values.extend([diff.before.evidence.clone(), diff.after.evidence.clone()]);
                }
                values
            }
            ArtifactTargetV2::CodeArtifact(value) => {
                BTreeSet::from([value.revision.evidence.clone()])
            }
            ArtifactTargetV2::Revision(value) => BTreeSet::from([value.evidence.clone()]),
            ArtifactTargetV2::Operation(value) => BTreeSet::from([value.clone()]),
            _ => BTreeSet::new(),
        }
    }
    pub fn agent_dependencies(&self) -> Result<BTreeSet<AgentObjectRef>> {
        self.validate()?;
        Ok(match &self.target {
            ArtifactTargetV2::AgentSession(value) => BTreeSet::from([value.session_snapshot]),
            ArtifactTargetV2::AgentToolArtifact(value) => {
                BTreeSet::from([value.session_snapshot, value.tool_snapshot])
            }
            ArtifactTargetV2::AgentToolText(value) => BTreeSet::from([
                value.tool.session_snapshot,
                value.tool.tool_snapshot,
                value.content,
            ]),
            _ => BTreeSet::new(),
        })
    }
}
