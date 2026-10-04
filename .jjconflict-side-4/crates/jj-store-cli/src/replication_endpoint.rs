//! Native RPC implementation of the transport-neutral replication endpoint.
use async_trait::async_trait;
use jj_store_model as m;
use jj_store_object::{Download, StagedUpload, WriteOutcome};
use jj_store_replication::{EndpointFailure, EndpointResult, ReplicationEndpoint};
use jj_store_rpc::client::NativeClient;
use tonic::Code;

#[derive(Clone)]
pub(crate) struct RpcEndpoint(pub(crate) NativeClient);

fn endpoint_error(operation: &'static str, status: tonic::Status) -> EndpointFailure {
    if matches!(
        status.code(),
        Code::Cancelled
            | Code::Unknown
            | Code::DeadlineExceeded
            | Code::ResourceExhausted
            | Code::Aborted
            | Code::Internal
            | Code::Unavailable
    ) {
        EndpointFailure::transient(operation)
    } else {
        EndpointFailure::permanent(operation)
    }
}

#[async_trait]
impl ReplicationEndpoint for RpcEndpoint {
    fn repository(&self) -> m::RepositoryId {
        self.0.repository()
    }

    async fn negotiate(
        &self,
        request: &jj_store_replication::NegotiateReplicaV1,
    ) -> EndpointResult<jj_store_replication::ReplicaNegotiationV1> {
        self.0
            .negotiate_replica(request)
            .await
            .map_err(|error| endpoint_error("negotiate", error))
    }

    async fn feed(
        &self,
        request: &jj_store_replication::FeedRequestV1,
    ) -> EndpointResult<jj_store_replication::FeedResponseV1> {
        self.0
            .committed_feed(request)
            .await
            .map_err(|error| endpoint_error("feed", error))
    }

    async fn offer(
        &self,
        request: &jj_store_replication::OfferOriginsRequestV1,
    ) -> EndpointResult<jj_store_replication::OfferOriginsResponseV1> {
        self.0
            .offer_origins(request)
            .await
            .map_err(|error| endpoint_error("offer", error))
    }

    async fn closure(
        &self,
        request: &jj_store_replication::OriginClosureRequestV1,
    ) -> EndpointResult<jj_store_replication::OriginClosureResponseV1> {
        self.0
            .origin_closure(request)
            .await
            .map_err(|error| endpoint_error("closure", error))
    }

    async fn read_native(&self, key: m::ObjectKey) -> EndpointResult<Download> {
        self.0
            .read(key)
            .await
            .map_err(|error| endpoint_error("read native", error))
    }

    async fn write_native(
        &self,
        key: m::ObjectKey,
        upload: &StagedUpload,
    ) -> EndpointResult<WriteOutcome> {
        self.0
            .upload(key, upload)
            .await
            .map_err(|error| endpoint_error("write native", error))
    }

    async fn read_control(
        &self,
        reference: jj_store_wal::records::RecordRef,
    ) -> EndpointResult<Vec<u8>> {
        self.0
            .read_control_record(reference)
            .await
            .map_err(|error| endpoint_error("read control", error))
    }

    async fn write_control(
        &self,
        reference: jj_store_wal::records::RecordRef,
        bytes: Vec<u8>,
    ) -> EndpointResult<WriteOutcome> {
        self.0
            .write_control_record(reference, bytes)
            .await
            .map_err(|error| endpoint_error("write control", error))
    }

    async fn apply(
        &self,
        request: &jj_store_replication::ApplyOriginsTransportRequestV1,
    ) -> EndpointResult<jj_store_replication::ApplyOriginsTransportResponseV1> {
        self.0
            .apply_origins(request)
            .await
            .map_err(|error| endpoint_error("apply", error))
    }
}
