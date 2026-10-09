//! Shared ReviewStore projection and cross-object validation without publication.
use super::*;
use jj_agent::AgentObjectRef;
const MAX_REVIEW_DEPENDENCIES: usize = 16_384;

/// Review-owned closure with direct external references. Each adapter must
/// verify the full external native/AgentStore closure before selecting a root.
pub struct ReviewDependencyClosure {
    pub objects: BTreeSet<ReviewObjectRef>,
    pub agent_objects: BTreeSet<AgentObjectRef>,
    pub native: BTreeSet<ObjectKey>,
    pub code_evidence: BTreeSet<jj_store_model::portable::CodeEvidenceRefV1>,
}

/// Stages immutable records only. Adapters own authorization, external evidence,
/// stable receipt replay, unknown outcomes and selection/publication.
pub struct ReviewProjector<'a> {
    pub(super) io: &'a dyn ReviewObjectIo,
}
impl<'a> ReviewProjector<'a> {
    pub fn new(io: &'a dyn ReviewObjectIo) -> Self {
        Self { io }
    }
    pub async fn stage_command(
        &self,
        store: &dyn ReviewSnapshotRead,
        request: &ReviewCommandRequest,
        accepted_at_millis: i64,
    ) -> std::result::Result<review_commands::AppliedReviewCommand, ReviewCommandError> {
        if store.repository() != self.io.repository() {
            return Err(ReviewCommandError::Invalid("review store repository"));
        }
        request.validate()?;
        if accepted_at_millis < 0 {
            return Err(ReviewCommandError::Invalid("review command time"));
        }
        self.apply_review_command(store, request, accepted_at_millis)
            .await
    }
    pub async fn audit_root(&self, root: &ReviewRootV1) -> Result<ReviewDependencyClosure> {
        root.validate().map_err(review::review_error)?;
        let closure = self
            .audit_dependencies(root.dependencies().map_err(review::review_error)?)
            .await?;
        self.audit_root_state(root, &closure.objects).await?;
        Ok(closure)
    }
    pub(super) async fn audit_root_state(
        &self,
        root: &ReviewRootV1,
        objects: &BTreeSet<ReviewObjectRef>,
    ) -> Result<()> {
        self.audit_review_root(root, objects).await?;
        self.audit_review_threads(objects).await
    }
    pub(super) async fn read_review_object(
        &self,
        reference: ReviewObjectRef,
    ) -> Result<ReviewObjectV1> {
        let object = self.io.read_review_object(reference).await?;
        if object.kind() != reference.kind {
            return Err(Error::Corrupt("review object kind"));
        }
        object.validate().map_err(review::review_error)?;
        if let ReviewObjectV1::Anchor(anchor) = &object
            && anchor.repository_id != self.io.repository()
        {
            return Err(Error::Corrupt("review anchor repository"));
        }
        Ok(object)
    }
    pub(super) async fn stage_review_object(
        &self,
        object: &ReviewObjectV1,
    ) -> Result<ReviewObjectRef> {
        self.io.stage_review_object(object).await
    }
    pub(super) async fn audit_dependencies(
        &self,
        mut pending: Vec<ReviewObjectRef>,
    ) -> Result<ReviewDependencyClosure> {
        let mut objects = BTreeSet::new();
        let mut native = BTreeSet::new();
        let mut agent = BTreeSet::new();
        let mut code_evidence = BTreeSet::new();
        while let Some(reference) = pending.pop() {
            if !objects.insert(reference) {
                continue;
            }
            if objects.len() > MAX_REVIEW_DEPENDENCIES {
                return Err(Error::Limit("review dependency closure"));
            }
            let object = self.read_review_object(reference).await?;
            code_evidence.extend(object.code_evidence().map_err(review::review_error)?);
            native.extend(object.native_dependencies().map_err(review::review_error)?);
            agent.extend(object.agent_dependencies().map_err(review::review_error)?);
            pending.extend(object.review_dependencies().map_err(review::review_error)?);
        }
        let agent_objects = agent;
        Ok(ReviewDependencyClosure {
            objects,
            agent_objects,
            native,
            code_evidence,
        })
    }
}
