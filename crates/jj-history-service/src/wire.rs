//! Bounded versioned JSON text keeps u64/i64 tokens intact through Tauri's JS bridge.
use crate::{Error, Result};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::io::{self, Write};

pub const VERSION: u32 = 1;
pub const MAX_REQUEST_BYTES: usize = 128 * 1024;
pub const MAX_RESPONSE_BYTES: usize = 16 * 1024 * 1024;

#[derive(Deserialize, Serialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Frame<T> {
    #[schemars(range(min = 1, max = 1))]
    pub version: u32,
    pub data: T,
}
pub fn decode<T: DeserializeOwned>(input: &str) -> Result<T> {
    if input.len() > MAX_REQUEST_BYTES {
        return Err(Error::Limit);
    }
    let frame: Frame<T> = serde_json::from_str(input).map_err(|_| Error::Invalid)?;
    if frame.version != VERSION {
        return Err(Error::Invalid);
    }
    Ok(frame.data)
}
pub fn encode<T: Serialize>(data: T) -> Result<String> {
    let mut writer = Bounded(Vec::new());
    serde_json::to_writer(
        &mut writer,
        &Frame {
            version: VERSION,
            data,
        },
    )
    .map_err(|error| {
        if error.is_io() {
            Error::Limit
        } else {
            Error::Unavailable
        }
    })?;
    String::from_utf8(writer.0).map_err(|_| Error::Unavailable)
}
struct Bounded(Vec<u8>);
impl Write for Bounded {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > MAX_RESPONSE_BYTES.saturating_sub(self.0.len()) {
            return Err(io::Error::other("history response byte limit"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
