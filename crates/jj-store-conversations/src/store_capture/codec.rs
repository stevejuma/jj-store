use super::EntryV1;
use crate::capture::CaptureError;

/// Bound for private recovery records, including byte-array JSON expansion.
pub const MAX_JOURNAL_BYTES: usize = 8 * 1024 * 1024;

impl EntryV1 {
    /// Validate and encode the private schema-1 DTO, without staging repository objects.
    pub fn encode(&self) -> Result<Vec<u8>, CaptureError> {
        self.validate()?;
        struct Bounded(Vec<u8>);
        impl std::io::Write for Bounded {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                if self.0.len().saturating_add(bytes.len()) > MAX_JOURNAL_BYTES {
                    return Err(std::io::Error::other("capture journal limit"));
                }
                self.0.extend_from_slice(bytes);
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut output = Bounded(Vec::new());
        serde_json::to_writer(&mut output, self).map_err(|_| CaptureError::Limit)?;
        Ok(output.0)
    }
    /// Decode bounded bytes directly into typed full-width Rust values.
    pub fn decode(bytes: &[u8]) -> Result<Self, CaptureError> {
        if bytes.len() > MAX_JOURNAL_BYTES {
            return Err(CaptureError::Limit);
        }
        let value: Self = serde_json::from_slice(bytes).map_err(|_| CaptureError::Invalid)?;
        value.validate()?;
        Ok(value)
    }
}
