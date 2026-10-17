//! Storage-independent primitives shared by the pinned read adapters.
mod accounting;
mod error;
mod limits;
pub mod workers;

pub use accounting::{Cancellation, Stats};
pub use error::{Diagnostic, Error, Result};
#[cfg(feature = "native")]
pub use limits::PredicateLimits;
pub use limits::{ContentLimits, GraphLimits, MetadataLimits};

pub(crate) const QUERY_CACHE_PROFILE: &str = "jj-native-0.45.1-v1";
