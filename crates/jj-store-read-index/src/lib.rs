//! Disposable, source-qualified secondary indexes. No repository authority or credentials.
#![forbid(unsafe_code)]
mod codec;
mod index;
mod storage;
pub use index::{Index, Row, Selection};
pub use storage::{Key, Stats, Store};

pub const MAX_ROWS: usize = 200_000;
pub const MAX_BYTES: usize = 64 * 1024 * 1024;
pub const SCHEMA: u32 = 1;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("derived index capacity exceeded")]
    Limit,
    #[error("invalid derived index")]
    Invalid,
    #[error("derived index unavailable")]
    Unavailable,
}
pub type Result<T> = std::result::Result<T, Error>;
