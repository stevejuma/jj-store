//! Explicit capacity before growing any serialized derived payload.
use crate::{Error, MAX_BYTES, Result};
use std::io::Write;
struct Output(Vec<u8>);
impl Write for Output {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.0.len().saturating_add(bytes.len()) > MAX_BYTES {
            return Err(std::io::Error::other("derived payload limit"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
pub(crate) fn encode<T: serde::Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut output = Output(vec![]);
    serde_json::to_writer(&mut output, value).map_err(|_| Error::Limit)?;
    Ok(output.0)
}
