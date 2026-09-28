//! Typed ReviewStore object staging and dependency verification.
use super::*;
use jj_agent::AgentObjectRef;
use jj_review::{Error as ReviewError, ReviewObjectId};

const MAX_REVIEW_DEPENDENCIES: usize = 16_384;

pub(super) struct ReviewDependencyClosure {
    pub objects: BTreeSet<ReviewObjectRef>,
    pub native: BTreeSet<ObjectKey>,
}

impl Coordinator {
    pub(super) async fn read_review_object(
        &self,
        reference: ReviewObjectRef,
    ) -> Result<ReviewObjectV1> {
        let object: ReviewObjectV1 = self
            .records
            .get(review_record(reference), RecordKind::ReviewObject)
            .await?;
        if object.kind() != reference.kind {
            return Err(Error::Corrupt("review object kind"));
        }
        object.validate().map_err(review_error)?;
        Ok(object)
    }

    pub async fn stage_review_object(&self, object: &ReviewObjectV1) -> Result<ReviewObjectRef> {
        let review = object.review_dependencies().map_err(review_error)?;
        let mut native = object.native_dependencies().map_err(review_error)?;
        let agent = object.agent_dependencies().map_err(review_error)?;
        native.extend(self.audit_review_dependencies(review).await?.native);
        self.audit_review_agent_dependencies(agent).await?;
        self.verify_delta(None, native).await?;
        let reference = self
            .records
            .put(RecordKind::ReviewObject, None, object)
            .await?;
        Ok(ReviewObjectRef::new(
            object.kind(),
            ReviewObjectId::new(*reference.digest.as_bytes()),
        ))
    }

    pub async fn stage_review_root(&self, root: &ReviewRootV1) -> Result<RecordRef> {
        let closure = self
            .audit_review_dependencies(root.dependencies().map_err(review_error)?)
            .await?;
        self.audit_review_root(root, &closure.objects).await?;
        self.audit_review_threads(&closure.objects).await?;
        self.verify_delta(None, closure.native).await?;
        self.records.put(RecordKind::ReviewRoot, None, root).await
    }

    pub(super) async fn read_review_root_dependencies(
        &self,
        reference: RecordRef,
    ) -> Result<(ReviewRootV1, BTreeSet<ObjectKey>)> {
        let (root, closure) = self.read_review_root_closure(reference).await?;
        Ok((root, closure.native))
    }

    pub(super) async fn read_review_root_closure(
        &self,
        reference: RecordRef,
    ) -> Result<(ReviewRootV1, ReviewDependencyClosure)> {
        let root: ReviewRootV1 = self.records.get(reference, RecordKind::ReviewRoot).await?;
        root.validate().map_err(review_error)?;
        let closure = self
            .audit_review_dependencies(root.dependencies().map_err(review_error)?)
            .await?;
        self.audit_review_root(&root, &closure.objects).await?;
        self.audit_review_threads(&closure.objects).await?;
        Ok((root, closure))
    }

    async fn audit_review_dependencies(
        &self,
        mut pending: Vec<ReviewObjectRef>,
    ) -> Result<ReviewDependencyClosure> {
        let mut objects = BTreeSet::new();
        let mut native = BTreeSet::new();
        let mut agent = BTreeSet::new();
        while let Some(reference) = pending.pop() {
            if !objects.insert(reference) {
                continue;
            }
            if objects.len() > MAX_REVIEW_DEPENDENCIES {
                return Err(Error::Limit("review dependency closure"));
            }
            let object = self.read_review_object(reference).await?;
            native.extend(object.native_dependencies().map_err(review_error)?);
            agent.extend(object.agent_dependencies().map_err(review_error)?);
            pending.extend(object.review_dependencies().map_err(review_error)?);
        }
        self.audit_review_agent_dependencies(agent).await?;
        Ok(ReviewDependencyClosure { objects, native })
    }

    async fn audit_review_agent_dependencies(
        &self,
        mut pending: BTreeSet<AgentObjectRef>,
    ) -> Result<()> {
        let mut seen = BTreeSet::new();
        while let Some(reference) = pending.pop_first() {
            if !seen.insert(reference) {
                continue;
            }
            if seen.len() > MAX_REVIEW_DEPENDENCIES {
                return Err(Error::Limit("review agent dependency closure"));
            }
            let object = self.read_agent_object(reference).await?;
            pending.extend(object.dependencies().map_err(super::agent::agent_error)?);
        }
        Ok(())
    }
}

fn review_record(reference: ReviewObjectRef) -> RecordRef {
    RecordRef {
        kind: RecordKind::ReviewObject,
        digest: RecordDigest::new(*reference.id.as_bytes()),
        attempt: None,
    }
}

pub(super) fn review_error(error: ReviewError) -> Error {
    match error {
        ReviewError::Limit => Error::Limit("review model"),
        ReviewError::Invalid(_) | ReviewError::Encoding(_) => Error::Corrupt("review model"),
    }
}
