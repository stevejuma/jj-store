//! Stable transport DTOs are owned by the WAL publication boundary so service
//! transports can consume them without depending on worker orchestration.
pub use jj_store_wal::publication::{
    ApplyOriginsTransportRequestV1, ApplyOriginsTransportResponseV1, FeedRequestV1, FeedResponseV1,
    NATIVE_PROFILE, NegotiateReplicaV1, ORIGIN_SCHEMA, OfferOriginsRequestV1,
    OfferOriginsResponseV1, OriginClosureRequestV1, OriginClosureResponseV1, ReplicaCapabilitiesV1,
    ReplicaNegotiationV1, SEMANTIC_REDUCER,
};
