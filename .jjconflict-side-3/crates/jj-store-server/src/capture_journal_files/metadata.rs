use super::*;
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Stamp([i128; 6]);
pub(super) fn id(metadata: &Metadata) -> Result<[u64; 2]> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        Ok([metadata.dev(), metadata.ino()])
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        Err(Error::Unavailable)
    }
}
pub(super) fn stamp(metadata: &Metadata) -> Result<Stamp> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        Ok(Stamp([
            metadata.dev() as i128,
            metadata.ino() as i128,
            metadata.len() as i128,
            metadata.mtime() as i128 * 1_000_000_000 + metadata.mtime_nsec() as i128,
            metadata.ctime() as i128 * 1_000_000_000 + metadata.ctime_nsec() as i128,
            metadata.mode() as i128,
        ]))
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        Err(Error::Unavailable)
    }
}
