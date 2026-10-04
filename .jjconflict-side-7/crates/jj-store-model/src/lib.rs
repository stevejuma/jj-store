//! Storage-independent identifiers and integrity framing for native JJ objects.
#![forbid(unsafe_code)]

pub mod envelope;
pub mod ids;
pub mod portable;
pub use ids::*;
