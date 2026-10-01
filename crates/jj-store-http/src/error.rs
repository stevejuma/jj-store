use crate::*;
use axum::response::IntoResponse;
use jj_store_service::{index_tasks as tasks, pagination as p, transport};

#[derive(Debug)]
pub struct Error {
    status: StatusCode,
    code: &'static str,
    snapshot: Option<Box<reads::ReadSnapshot>>,
    heads: Option<Box<reads::HeadSnapshot>>,
    length: Option<u64>,
    search_status: Option<Box<jj_store_tantivy::paging::Status>>,
}
impl Error {
    pub fn new(status: StatusCode, code: &'static str) -> Self {
        Self {
            status,
            code,
            snapshot: None,
            heads: None,
            length: None,
            search_status: None,
        }
    }
    pub fn invalid() -> Self {
        Self::new(StatusCode::BAD_REQUEST, "invalid_request")
    }
    pub fn denied() -> Self {
        Self::new(StatusCode::FORBIDDEN, "access_denied")
    }
    pub fn limit() -> Self {
        Self::new(StatusCode::PAYLOAD_TOO_LARGE, "resource_limit")
    }
    pub fn busy() -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, "busy")
    }
    pub fn unavailable() -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, "unavailable")
    }
    pub fn timeout() -> Self {
        Self::new(StatusCode::GATEWAY_TIMEOUT, "deadline_exceeded")
    }
    pub fn range(length: u64) -> Self {
        Self {
            length: Some(length),
            ..Self::new(StatusCode::RANGE_NOT_SATISFIABLE, "invalid_range")
        }
    }
    pub fn heads(heads: reads::HeadSnapshot) -> Self {
        Self {
            heads: Some(Box::new(heads)),
            ..Self::new(StatusCode::CONFLICT, "select_operation")
        }
    }
    pub fn rebuild(snapshot: reads::ReadSnapshot) -> Self {
        Self {
            snapshot: Some(Box::new(snapshot)),
            ..Self::new(StatusCode::CONFLICT, "index_rebuild_required")
        }
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let value = crate::dto::ErrorBody {
            version: 1,
            code: self.code,
            snapshot: self.snapshot,
            heads: self.heads,
            search_status: self.search_status,
        };
        let (status, bytes) = match encode(&value, 64 * 1024) {
            Ok(bytes) => (self.status, bytes),
            Err(_) => (
                StatusCode::PAYLOAD_TOO_LARGE,
                b"{\"version\":1,\"code\":\"resource_limit\"}".to_vec(),
            ),
        };
        let mut response =
            (status, [(header::CONTENT_TYPE, "application/json")], bytes).into_response();
        if let Some(length) = self.length {
            response.headers_mut().insert(
                header::CONTENT_RANGE,
                format!("bytes */{length}").parse().unwrap(),
            );
        }
        if self.code == "busy" {
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, "1".parse().unwrap());
        }
        if self.status == StatusCode::UNAUTHORIZED {
            response
                .headers_mut()
                .insert(header::WWW_AUTHENTICATE, "Bearer".parse().unwrap());
        }
        response
    }
}
impl From<q::Error> for Error {
    fn from(e: q::Error) -> Self {
        use q::Error as E;
        match e {
            E::Denied => Self::denied(),
            E::Busy => Self::busy(),
            E::Limit | E::IndexBudget => Self::limit(),
            E::NotCommitted | E::NotFound => Self::new(StatusCode::NOT_FOUND, "not_found"),
            E::Invalid | E::NotDirectory | E::Range => Self::invalid(),
            E::EpochChanged => Self::new(StatusCode::CONFLICT, "epoch_changed"),
            E::IndexRebuildRequired => Self::new(StatusCode::CONFLICT, "index_rebuild_required"),
            E::Canceled => Self::new(StatusCode::REQUEST_TIMEOUT, "canceled"),
            E::Corrupt => Self::new(StatusCode::SERVICE_UNAVAILABLE, "corrupt_history"),
            E::Incomplete => Self::new(StatusCode::SERVICE_UNAVAILABLE, "incomplete_history"),
            E::Unavailable => Self::unavailable(),
        }
    }
}
impl From<reads::Error> for Error {
    fn from(e: reads::Error) -> Self {
        match e {
            reads::Error::Denied => Self::denied(),
            reads::Error::NotCommitted => Self::new(StatusCode::NOT_FOUND, "not_found"),
            reads::Error::EpochChanged => Self::new(StatusCode::CONFLICT, "epoch_changed"),
            reads::Error::Limit => Self::limit(),
            reads::Error::Unavailable => Self::unavailable(),
        }
    }
}
impl From<p::Error> for Error {
    fn from(e: p::Error) -> Self {
        match e {
            p::Error::Denied => Self::denied(),
            p::Error::Busy => Self::busy(),
            p::Error::Limit => Self::limit(),
            p::Error::Unavailable => Self::unavailable(),
            p::Error::InvalidCursor => Self::new(StatusCode::BAD_REQUEST, "invalid_cursor"),
            p::Error::CursorReset => Self::new(StatusCode::CONFLICT, "cursor_reset"),
            p::Error::IndexRebuildRequired { snapshot } => Self::rebuild(*snapshot),
            p::Error::Read(e) => e.into(),
            p::Error::Query(e) => e.into(),
        }
    }
}
impl From<jj_store_service::collaboration::Error> for Error {
    fn from(error: jj_store_service::collaboration::Error) -> Self {
        use jj_store_service::collaboration::Error as E;
        match error {
            E::Denied => Self::denied(),
            E::InvalidCursor => Self::new(StatusCode::BAD_REQUEST, "invalid_cursor"),
            E::CursorReset => Self::new(StatusCode::CONFLICT, "cursor_reset"),
            E::NotFound => Self::new(StatusCode::NOT_FOUND, "not_found"),
            E::Limit => Self::limit(),
            E::Busy => Self::busy(),
            E::Corrupt => Self::new(StatusCode::SERVICE_UNAVAILABLE, "corrupt_history"),
            E::Unavailable => Self::unavailable(),
        }
    }
}

impl From<jj_store_service::review_mutations::Error> for Error {
    fn from(error: jj_store_service::review_mutations::Error) -> Self {
        use jj_store_service::review_mutations::Error as E;
        match error {
            E::Denied => Self::denied(),
            E::Stale => Self::new(StatusCode::CONFLICT, "stale_review"),
            E::IntentConflict => Self::new(StatusCode::CONFLICT, "intent_conflict"),
            E::ThreadResolved => Self::new(StatusCode::CONFLICT, "thread_resolved"),
            E::ThreadOpen => Self::new(StatusCode::CONFLICT, "thread_open"),
            E::NotFound => Self::new(StatusCode::NOT_FOUND, "not_found"),
            E::Invalid => Self::invalid(),
            E::Corrupt => Self::new(StatusCode::SERVICE_UNAVAILABLE, "corrupt_receipt"),
            E::Limit => Self::limit(),
            E::Unavailable => Self::unavailable(),
        }
    }
}
impl From<transport::Error> for Error {
    fn from(e: transport::Error) -> Self {
        match e {
            transport::Error::Denied => Self::denied(),
            transport::Error::Unsupported => Self::new(StatusCode::CONFLICT, "unsupported_profile"),
            transport::Error::Limit => Self::limit(),
            transport::Error::Invalid => Self::invalid(),
            _ => Self::unavailable(),
        }
    }
}
impl From<tasks::Error> for Error {
    fn from(e: tasks::Error) -> Self {
        match e {
            tasks::Error::Denied => Self::denied(),
            tasks::Error::Busy => Self::busy(),
            tasks::Error::Limit => Self::limit(),
            tasks::Error::Unavailable => Self::unavailable(),
            tasks::Error::EpochChanged => Self::new(StatusCode::CONFLICT, "epoch_changed"),
            tasks::Error::Reset => Self::new(StatusCode::GONE, "task_reset"),
            tasks::Error::Read(e) => e.into(),
        }
    }
}

impl From<events::Error> for Error {
    fn from(value: events::Error) -> Self {
        match value {
            events::Error::Denied => Self::denied(),
            events::Error::InvalidCursor => Self::new(StatusCode::BAD_REQUEST, "invalid_cursor"),
            events::Error::Limit => Self::limit(),
            events::Error::Unavailable => Self::unavailable(),
        }
    }
}

impl From<jj_store_service::mutations::Error> for Error {
    fn from(error: jj_store_service::mutations::Error) -> Self {
        use jj_store_service::{commands::Error as C, mutations::Error as E};
        match error {
            E::Denied => Self::denied(),
            E::Protected => Self::new(StatusCode::CONFLICT, "protected_transition"),
            E::Stale => Self::new(StatusCode::CONFLICT, "stale_snapshot"),
            E::IntentConflict => Self::new(StatusCode::CONFLICT, "intent_conflict"),
            E::Limit => Self::limit(),
            E::Invalid => Self::invalid(),
            E::Corrupt => Self::new(StatusCode::SERVICE_UNAVAILABLE, "corrupt_receipt"),
            E::Unavailable => Self::unavailable(),
            E::Preparation(C::Query(error)) => error.into(),
            E::Preparation(C::Stale) => Self::new(StatusCode::CONFLICT, "stale_snapshot"),
            E::Preparation(C::MultipleHeads) => {
                Self::new(StatusCode::CONFLICT, "reconciliation_required")
            }
            E::Preparation(C::InvalidUpload) => Self::new(StatusCode::CONFLICT, "invalid_upload"),
            E::Preparation(C::StaleConflict) => Self::new(StatusCode::CONFLICT, "stale_conflict"),
            E::Preparation(C::StaleBookmark) => Self::new(StatusCode::CONFLICT, "stale_bookmark"),
            E::Preparation(C::Root) => Self::new(StatusCode::CONFLICT, "protected_transition"),
            E::Preparation(C::Hidden) => Self::new(StatusCode::NOT_FOUND, "not_found"),
            E::Preparation(C::Invalid) => Self::invalid(),
            E::Preparation(C::NoChanges) => Self::new(StatusCode::CONFLICT, "no_changes"),
        }
    }
}

impl From<jj_store_service::uploads::Error> for Error {
    fn from(value: jj_store_service::uploads::Error) -> Self {
        use jj_store_service::uploads::Error as E;
        match value {
            E::Denied => Self::denied(),
            E::Stale => Self::new(StatusCode::CONFLICT, "stale_snapshot"),
            E::Invalid => Self::invalid(),
            E::InvalidUpload => Self::new(StatusCode::CONFLICT, "invalid_upload"),
            E::Limit => Self::limit(),
            E::Busy => Self::busy(),
            E::Corrupt => Self::new(StatusCode::SERVICE_UNAVAILABLE, "corrupt_upload"),
            E::Unavailable => Self::unavailable(),
        }
    }
}

impl From<jj_store_tantivy::paging::Error> for Error {
    fn from(value: jj_store_tantivy::paging::Error) -> Self {
        use jj_store_adapters::Error as A;
        use jj_store_tantivy::paging::Error as E;
        match value {
            E::Invalid => Self::invalid(),
            E::InvalidCursor => Self::new(StatusCode::BAD_REQUEST, "invalid_cursor"),
            E::CursorReset => Self::new(StatusCode::CONFLICT, "cursor_reset"),
            E::EpochChanged => Self::new(StatusCode::CONFLICT, "epoch_changed"),
            E::NotReady => Self::new(StatusCode::SERVICE_UNAVAILABLE, "search_not_ready"),
            E::IndexLag(status) => Self {
                search_status: Some(status),
                ..Self::new(StatusCode::CONFLICT, "index_lag")
            },
            E::SelectHead(heads) => Self::heads(*heads),
            E::Limit | E::Adapter(A::Limit) => Self::limit(),
            E::Busy | E::Adapter(A::Busy) => Self::busy(),
            E::Unavailable | E::Adapter(A::Unavailable | A::CursorConflict | A::Projection) => {
                Self::unavailable()
            }
            E::Adapter(A::Denied) => Self::denied(),
            E::Adapter(A::Invalid) => Self::invalid(),
            E::Adapter(A::Corrupt) => Self::new(StatusCode::SERVICE_UNAVAILABLE, "corrupt_index"),
            E::Adapter(A::Canceled) => Self::new(StatusCode::REQUEST_TIMEOUT, "canceled"),
            E::Read(e) => e.into(),
            E::Native(e) => e.into(),
        }
    }
}

impl From<jj_store_service::work::Error> for Error {
    fn from(error: jj_store_service::work::Error) -> Self {
        use jj_store_service::work::Error as E;
        match error {
            E::Denied => Self::denied(),
            E::Stale => Self::new(StatusCode::CONFLICT, "stale_work"),
            E::IntentConflict => Self::new(StatusCode::CONFLICT, "intent_conflict"),
            E::NotFound => Self::new(StatusCode::NOT_FOUND, "not_found"),
            E::Invalid | E::InvalidCursor => Self::invalid(),
            E::CursorReset => Self::new(StatusCode::CONFLICT, "cursor_reset"),
            E::Busy => Self::busy(),
            E::Corrupt => Self::new(StatusCode::SERVICE_UNAVAILABLE, "corrupt_work"),
            E::Limit => Self::limit(),
            E::Unavailable => Self::unavailable(),
            E::ReplicationUnsupported => {
                Self::new(StatusCode::CONFLICT, "work_replication_unsupported")
            }
        }
    }
}

impl From<jj_store_service::collab_mutations::Error> for Error {
    fn from(error: jj_store_service::collab_mutations::Error) -> Self {
        use jj_store_service::collab_mutations::Error as E;
        match error {
            E::Denied => Self::denied(),
            E::Stale => Self::new(StatusCode::CONFLICT, "stale_collaboration"),
            E::IntentConflict => Self::new(StatusCode::CONFLICT, "intent_conflict"),
            E::Unsupported => Self::new(
                StatusCode::CONFLICT,
                "collaboration_replication_unsupported",
            ),
            E::NotFound => Self::new(StatusCode::NOT_FOUND, "not_found"),
            E::Invalid => Self::invalid(),
            E::Corrupt => Self::new(StatusCode::SERVICE_UNAVAILABLE, "corrupt_collaboration"),
            E::Limit => Self::limit(),
            E::Unavailable => Self::unavailable(),
        }
    }
}
impl From<jj_store_service::collab_pages::Error> for Error {
    fn from(error: jj_store_service::collab_pages::Error) -> Self {
        use jj_store_service::collab_pages::Error as E;
        match error {
            E::InvalidCursor => Self::new(StatusCode::BAD_REQUEST, "invalid_cursor"),
            E::CursorReset => Self::new(StatusCode::CONFLICT, "cursor_reset"),
            E::Busy => Self::busy(),
            E::Limit => Self::limit(),
            E::Unavailable => Self::unavailable(),
            E::Read(error) => error.into(),
        }
    }
}
