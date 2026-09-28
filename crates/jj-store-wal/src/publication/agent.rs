//! Typed AgentStore object staging and dependency verification.
use super::*;
use jj_agent::{AgentObjectId, Error as AgentError};

const MAX_AGENT_DEPENDENCIES: usize = 4096;

impl Coordinator {
    pub async fn stage_agent_object(&self, object: &AgentObjectV1) -> Result<AgentObjectRef> {
        let dependencies = object.dependencies().map_err(agent_error)?;
        self.audit_agent_dependencies(dependencies).await?;
        let reference = self
            .records
            .put(RecordKind::AgentObject, None, object)
            .await?;
        Ok(AgentObjectRef::new(
            object.kind(),
            AgentObjectId::new(*reference.digest.as_bytes()),
        ))
    }

    pub async fn stage_agent_root(&self, root: &AgentRootV1) -> Result<RecordRef> {
        let dependencies = root.dependencies().map_err(agent_error)?;
        self.audit_agent_dependencies(dependencies).await?;
        self.records.put(RecordKind::AgentRoot, None, root).await
    }

    pub async fn read_agent_root(&self, reference: RecordRef) -> Result<AgentRootV1> {
        let root: AgentRootV1 = self.records.get(reference, RecordKind::AgentRoot).await?;
        let dependencies = root.dependencies().map_err(agent_error)?;
        self.audit_agent_dependencies(dependencies).await?;
        Ok(root)
    }

    async fn audit_agent_dependencies(&self, mut pending: Vec<AgentObjectRef>) -> Result<()> {
        let mut seen = BTreeSet::new();
        while let Some(reference) = pending.pop() {
            if !seen.insert(reference) {
                continue;
            }
            if seen.len() > MAX_AGENT_DEPENDENCIES {
                return Err(Error::Limit("agent dependency closure"));
            }
            let record = RecordRef {
                kind: RecordKind::AgentObject,
                digest: RecordDigest::new(*reference.id.as_bytes()),
                attempt: None,
            };
            let object: AgentObjectV1 = self.records.get(record, RecordKind::AgentObject).await?;
            if object.kind() != reference.kind {
                return Err(Error::Corrupt("agent object kind"));
            }
            pending.extend(object.dependencies().map_err(agent_error)?);
        }
        Ok(())
    }
}

fn agent_error(error: AgentError) -> Error {
    match error {
        AgentError::Limit => Error::Limit("agent object"),
        AgentError::Invalid(_) | AgentError::Encoding(_) => Error::Corrupt("agent object"),
    }
}
