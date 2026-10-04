//! Review identities for actual retained agent records, independent of capture.
use crate::{ArtifactAnchorV2, ArtifactTargetV2, Error, Result, TextSelectionV1};
use jj_agent::{
    AgentObjectKind, AgentObjectRef, AgentObjectV1, AgentSessionId, AgentSessionSnapshotV1,
    ToolCallId, ToolInvocationSnapshotV1,
};
use jj_store_model::RepositoryId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PortableAgentSessionV2 {
    pub session_id: AgentSessionId,
    pub session_snapshot: AgentObjectRef,
}
impl PortableAgentSessionV2 {
    pub fn validate(&self) -> Result<()> {
        if self.session_snapshot.kind != AgentObjectKind::SessionSnapshot {
            return Err(Error::Invalid("portable agent session kind"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PortableAgentToolV2 {
    pub session_id: AgentSessionId,
    pub session_snapshot: AgentObjectRef,
    pub tool_call_id: ToolCallId,
    pub tool_snapshot: AgentObjectRef,
}
impl PortableAgentToolV2 {
    pub fn validate(&self) -> Result<()> {
        if self.session_snapshot.kind != AgentObjectKind::SessionSnapshot
            || self.tool_snapshot.kind != AgentObjectKind::ToolSnapshot
        {
            return Err(Error::Invalid("portable agent tool kind"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentToolContentPartV2 {
    Input,
    Output,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PortableAgentToolTextV2 {
    pub tool: PortableAgentToolV2,
    pub part: AgentToolContentPartV2,
    pub content: AgentObjectRef,
    pub text: TextSelectionV1,
}
impl PortableAgentToolTextV2 {
    pub fn validate(&self) -> Result<()> {
        self.tool.validate()?;
        if self.content.kind != AgentObjectKind::Content {
            return Err(Error::Invalid("portable agent content kind"));
        }
        self.text.validate()
    }
}

impl ArtifactAnchorV2 {
    /// Validate cross-object identities against exact, retained records.
    /// The provider must enforce repository envelopes, selected membership and
    /// reference/content identity in `read`. This pure check grants no access
    /// and does not establish retention or publication authority.
    pub fn validate_agent_records(
        &self,
        repository: RepositoryId,
        mut read: impl FnMut(AgentObjectRef) -> Result<AgentObjectV1>,
    ) -> Result<()> {
        self.validate()?;
        if self.repository_id != repository {
            return Err(Error::Invalid("portable agent repository"));
        }
        match &self.target {
            ArtifactTargetV2::AgentSession(value) => {
                session(&mut read, value.session_snapshot, value.session_id)?;
            }
            ArtifactTargetV2::AgentToolArtifact(value) => {
                tool(&mut read, value)?;
            }
            ArtifactTargetV2::AgentToolText(value) => {
                let selected = tool(&mut read, &value.tool)?;
                let content = match value.part {
                    AgentToolContentPartV2::Input => selected.input,
                    AgentToolContentPartV2::Output => selected.output,
                };
                if content != Some(value.content) {
                    return Err(Error::Invalid("portable tool content selection"));
                }
                let AgentObjectV1::Content(content) = record(&mut read, value.content)? else {
                    return Err(Error::Invalid("portable tool content record"));
                };
                value.text.validate_bytes(content.payload.bytes())?;
            }
            _ => return Err(Error::Invalid("anchor is not portable agent")),
        }
        Ok(())
    }
}

fn record(
    read: &mut impl FnMut(AgentObjectRef) -> Result<AgentObjectV1>,
    reference: AgentObjectRef,
) -> Result<AgentObjectV1> {
    let value = read(reference)?;
    value
        .validate()
        .map_err(|_| Error::Invalid("portable agent record"))?;
    if value.kind() != reference.kind {
        return Err(Error::Invalid("portable agent record kind"));
    }
    Ok(value)
}
fn session(
    read: &mut impl FnMut(AgentObjectRef) -> Result<AgentObjectV1>,
    reference: AgentObjectRef,
    id: AgentSessionId,
) -> Result<AgentSessionSnapshotV1> {
    let AgentObjectV1::SessionSnapshot(value) = record(read, reference)? else {
        return Err(Error::Invalid("portable agent session record"));
    };
    if value.session_id != id {
        return Err(Error::Invalid("portable agent session identity"));
    }
    Ok(value)
}
fn tool(
    read: &mut impl FnMut(AgentObjectRef) -> Result<AgentObjectV1>,
    anchor: &PortableAgentToolV2,
) -> Result<ToolInvocationSnapshotV1> {
    let selected = session(read, anchor.session_snapshot, anchor.session_id)?;
    if selected.tools.get(&anchor.tool_call_id) != Some(&anchor.tool_snapshot) {
        return Err(Error::Invalid("portable agent tool membership"));
    }
    let AgentObjectV1::ToolSnapshot(value) = record(read, anchor.tool_snapshot)? else {
        return Err(Error::Invalid("portable agent tool record"));
    };
    if value.session_id != anchor.session_id || value.tool_call_id != anchor.tool_call_id {
        return Err(Error::Invalid("portable agent tool identity"));
    }
    Ok(value)
}
