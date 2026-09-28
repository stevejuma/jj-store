//! Typed AgentStore object staging and dependency verification.
use super::*;
use jj_agent::{AgentObjectId, Error as AgentError};

const MAX_AGENT_DEPENDENCIES: usize = 4096;

struct AgentDependencyClosure {
    objects: BTreeSet<AgentObjectRef>,
    native: BTreeSet<ObjectKey>,
}

impl Coordinator {
    pub(super) async fn read_agent_object(
        &self,
        reference: AgentObjectRef,
    ) -> Result<AgentObjectV1> {
        let object: AgentObjectV1 = self
            .records
            .get(agent_record(reference), RecordKind::AgentObject)
            .await?;
        if object.kind() != reference.kind {
            return Err(Error::Corrupt("agent object kind"));
        }
        object.validate().map_err(agent_error)?;
        Ok(object)
    }

    pub async fn stage_agent_object(&self, object: &AgentObjectV1) -> Result<AgentObjectRef> {
        let dependencies = object.dependencies().map_err(agent_error)?;
        let mut native = object.native_dependencies().map_err(agent_error)?;
        native.extend(self.audit_agent_dependencies(dependencies).await?.native);
        self.verify_delta(None, native).await?;
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
        let closure = self.audit_agent_dependencies(dependencies).await?;
        self.verify_delta(None, closure.native).await?;
        self.records.put(RecordKind::AgentRoot, None, root).await
    }

    pub async fn read_agent_root(&self, reference: RecordRef) -> Result<AgentRootV1> {
        let (root, native, _) = self.read_agent_root_closure(reference).await?;
        self.verify_delta(None, native).await?;
        Ok(root)
    }

    pub(super) async fn read_agent_root_dependencies(
        &self,
        reference: RecordRef,
    ) -> Result<(AgentRootV1, BTreeSet<ObjectKey>)> {
        let (root, native, _) = self.read_agent_root_closure(reference).await?;
        Ok((root, native))
    }

    pub(super) async fn read_agent_root_closure(
        &self,
        reference: RecordRef,
    ) -> Result<(AgentRootV1, BTreeSet<ObjectKey>, BTreeSet<AgentObjectRef>)> {
        let root: AgentRootV1 = self.records.get(reference, RecordKind::AgentRoot).await?;
        let dependencies = root.dependencies().map_err(agent_error)?;
        let closure = self.audit_agent_dependencies(dependencies).await?;
        Ok((root, closure.native, closure.objects))
    }

    async fn audit_agent_dependencies(
        &self,
        mut pending: Vec<AgentObjectRef>,
    ) -> Result<AgentDependencyClosure> {
        let mut seen = BTreeSet::new();
        let mut native = BTreeSet::new();
        while let Some(reference) = pending.pop() {
            if !seen.insert(reference) {
                continue;
            }
            if seen.len() > MAX_AGENT_DEPENDENCIES {
                return Err(Error::Limit("agent dependency closure"));
            }
            let object = self.read_agent_object(reference).await?;
            native.extend(object.native_dependencies().map_err(agent_error)?);
            pending.extend(object.dependencies().map_err(agent_error)?);
        }
        Ok(AgentDependencyClosure {
            objects: seen,
            native,
        })
    }
}

fn agent_record(reference: AgentObjectRef) -> RecordRef {
    RecordRef {
        kind: RecordKind::AgentObject,
        digest: RecordDigest::new(*reference.id.as_bytes()),
        attempt: None,
    }
}

pub(super) fn agent_error(error: AgentError) -> Error {
    match error {
        AgentError::Limit => Error::Limit("agent object"),
        AgentError::Invalid(_) | AgentError::Encoding(_) => Error::Corrupt("agent object"),
    }
}
