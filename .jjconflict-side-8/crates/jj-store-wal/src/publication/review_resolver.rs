//! Root-pinned, read-only ReviewStore anchor placement.
use super::*;
use jj_review::{AnchorResolutionRequestV1, AnchorResolverV1, PlacementResultV1, ResolverBudgetV1};

impl ReviewStoreSnapshot {
    pub async fn resolve_anchor(
        &self,
        anchor: ReviewObjectRef,
        request: &AnchorResolutionRequestV1,
        budget: ResolverBudgetV1,
    ) -> Result<PlacementResultV1> {
        let ReviewObjectV1::Anchor(anchor) = self.read_object(anchor).await? else {
            return Err(Error::Corrupt("review resolver anchor kind"));
        };
        AnchorResolverV1::new(budget)
            .and_then(|resolver| resolver.resolve(&anchor, request))
            .map_err(super::review::review_error)
    }
}
