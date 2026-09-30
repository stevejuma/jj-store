use super::{
    ApplyOriginsOutcomeV1, OriginClosureV1, OriginEnvelopeV1, OriginFeedPageV1, OriginKeyV1,
    OriginOfferResultV1, OriginOfferV1,
};
use jj_store_model::{EpochId, PublicationId, ReplicaId, RepositoryId, RequestId, WalPosition};
use serde::{Deserialize, Serialize};

pub const NATIVE_PROFILE: &str = "jj-native-0.45.1-v1";
pub const ORIGIN_SCHEMA: u32 = 1;
pub const SEMANTIC_REDUCER: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplicaCapabilitiesV1 {
    pub schema_version: u32,
    pub native_profile: String,
    pub origin_schema: u32,
    pub semantic_reducer: u32,
}

impl Default for ReplicaCapabilitiesV1 {
    fn default() -> Self {
        Self {
            schema_version: 1,
            native_profile: NATIVE_PROFILE.to_owned(),
            origin_schema: ORIGIN_SCHEMA,
            semantic_reducer: SEMANTIC_REDUCER,
        }
    }
}

impl ReplicaCapabilitiesV1 {
    pub fn compatible_with(&self, peer: &Self) -> bool {
        self.schema_version == 1
            && peer.schema_version == 1
            && self.native_profile == peer.native_profile
            && self.origin_schema == peer.origin_schema
            && self.semantic_reducer == peer.semantic_reducer
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NegotiateReplicaV1 {
    pub schema_version: u32,
    pub repository: RepositoryId,
    pub peer: ReplicaId,
    pub capabilities: ReplicaCapabilitiesV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplicaNegotiationV1 {
    pub schema_version: u32,
    pub repository: RepositoryId,
    pub replica: ReplicaId,
    pub log_epoch: EpochId,
    pub feed_upper_bound: WalPosition,
    pub capabilities: ReplicaCapabilitiesV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeedRequestV1 {
    pub schema_version: u32,
    pub source_replica: ReplicaId,
    pub log_epoch: EpochId,
    pub after: WalPosition,
    pub upper_bound: WalPosition,
    pub limit: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeedResponseV1 {
    pub schema_version: u32,
    pub page: OriginFeedPageV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfferOriginsRequestV1 {
    pub schema_version: u32,
    pub offers: Vec<OriginOfferV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfferOriginsResponseV1 {
    pub schema_version: u32,
    pub results: Vec<OriginOfferResultV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginClosureRequestV1 {
    pub schema_version: u32,
    pub origin_key: OriginKeyV1,
    pub max_entries: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginClosureResponseV1 {
    pub schema_version: u32,
    pub closure: OriginClosureV1,
}

/// The transport never accepts a caller-authored principal. The authenticated
/// server session derives it and constructs the guarded publication request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApplyOriginsTransportRequestV1 {
    pub schema_version: u32,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub origins: Vec<OriginEnvelopeV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApplyOriginsTransportResponseV1 {
    pub schema_version: u32,
    pub outcome: ApplyOriginsOutcomeV1,
}
