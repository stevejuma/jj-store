//! Portable code provenance with explicit source profiles and retained evidence.
use crate::{
    AgentCheckpointFactsV1, AssociationGranularity, CodeAssociationFactsV1, CodeRelation, Error,
    Result,
};
use jj_store_model::portable::{CodeCommitIdV1, CodeEvidenceRefV1};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct CodeCommitEvidenceV2 {
    pub evidence: CodeEvidenceRefV1,
    pub commit: CodeCommitIdV1,
}
impl CodeCommitEvidenceV2 {
    pub fn validate(&self) -> Result<()> {
        if self.evidence.profile != self.commit.profile() {
            return Err(Error::Invalid("portable commit source profile"));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeAnchorV2 {
    pub operations: BTreeSet<CodeEvidenceRefV1>,
    pub commits: BTreeSet<CodeCommitEvidenceV2>,
}
impl CodeAnchorV2 {
    pub fn validate(&self) -> Result<()> {
        if self.operations.len() > 256 || self.commits.len() > 256 {
            return Err(Error::Limit);
        }
        for commit in &self.commits {
            commit.validate()?;
        }
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty() && self.commits.is_empty()
    }
    pub fn evidence(&self) -> BTreeSet<CodeEvidenceRefV1> {
        self.operations
            .iter()
            .cloned()
            .chain(self.commits.iter().map(|v| v.evidence.clone()))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CodeAssociationV2 {
    pub schema_version: u32,
    pub before: CodeAnchorV2,
    pub after: CodeAnchorV2,
    pub facts: CodeAssociationFactsV1,
}
impl CodeAssociationV2 {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        crate::codec::encode(self, 16 * 1024 * 1024)
    }
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let value: Self = crate::codec::decode(bytes, 16 * 1024 * 1024)?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<()> {
        self.before.validate()?;
        self.after.validate()?;
        self.facts.validate()?;
        let wrong_granularity = match self.facts.granularity {
            AssociationGranularity::Operation => {
                self.before.operations.is_empty() && self.after.operations.is_empty()
            }
            AssociationGranularity::Revision => {
                self.before.commits.is_empty() && self.after.commits.is_empty()
            }
            AssociationGranularity::SnapshotInterval => false,
            AssociationGranularity::File => self.facts.paths.is_empty(),
        };
        if self.schema_version != 2
            || (self.before.is_empty() && self.after.is_empty())
            || wrong_granularity
        {
            return Err(Error::Invalid("portable association"));
        }
        Ok(())
    }
    pub fn dependencies(&self) -> Result<BTreeSet<crate::AgentObjectRef>> {
        self.validate()?;
        self.facts.dependencies()
    }
    pub fn code_evidence(&self) -> Result<BTreeSet<CodeEvidenceRefV1>> {
        self.validate()?;
        Ok(self
            .before
            .evidence()
            .into_iter()
            .chain(self.after.evidence())
            .collect())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentCheckpointVersionV2 {
    pub schema_version: u32,
    pub before: CodeAnchorV2,
    pub after: CodeAnchorV2,
    pub code_relation: CodeRelation,
    pub facts: AgentCheckpointFactsV1,
}
impl AgentCheckpointVersionV2 {
    pub fn encode(&self) -> Result<Vec<u8>> {
        self.validate()?;
        crate::codec::encode(self, 16 * 1024 * 1024)
    }
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let value: Self = crate::codec::decode(bytes, 16 * 1024 * 1024)?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<()> {
        self.before.validate()?;
        self.after.validate()?;
        self.facts.validate()?;
        let valid = match self.code_relation {
            CodeRelation::Changed => {
                !self.before.is_empty() && !self.after.is_empty() && self.before != self.after
            }
            CodeRelation::NoChange => !self.before.is_empty() && self.before == self.after,
            CodeRelation::NotObserved => self.before.is_empty() || self.after.is_empty(),
        };
        if self.schema_version != 2 || !valid {
            return Err(Error::Invalid("portable checkpoint relation"));
        }
        Ok(())
    }
    pub fn dependencies(&self) -> Result<BTreeSet<crate::AgentObjectRef>> {
        self.validate()?;
        self.facts.dependencies()
    }
    pub fn code_evidence(&self) -> Result<BTreeSet<CodeEvidenceRefV1>> {
        self.validate()?;
        Ok(self
            .before
            .evidence()
            .into_iter()
            .chain(self.after.evidence())
            .collect())
    }
}
