use crate::{Error, Result, ReviewDigest};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ByteOffset(u64);
impl ByteOffset {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
    pub const fn get(self) -> u64 {
        self.0
    }
}
impl Serialize for ByteOffset {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}
impl<'de> Deserialize<'de> for ByteOffset {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        if value == "0" || (!value.starts_with('0') && value.bytes().all(|b| b.is_ascii_digit())) {
            value.parse().map(Self).map_err(D::Error::custom)
        } else {
            Err(D::Error::custom("noncanonical decimal byte offset"))
        }
    }
}
impl schemars::JsonSchema for ByteOffset {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ByteOffset".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type": "string", "pattern": "^(0|[1-9][0-9]*)$" })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TextEncoding {
    Utf8,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TextSelectionV1 {
    pub encoding: TextEncoding,
    pub start_byte: ByteOffset,
    pub end_byte: ByteOffset,
    pub selected_sha256: ReviewDigest,
}
impl TextSelectionV1 {
    pub fn validate(&self) -> Result<()> {
        if self.start_byte >= self.end_byte {
            return Err(Error::Invalid("empty or reversed text selection"));
        }
        Ok(())
    }
    pub fn validate_bytes(&self, bytes: &[u8]) -> Result<()> {
        self.validate()?;
        let start = usize::try_from(self.start_byte.get())
            .map_err(|_| Error::Invalid("text selection start"))?;
        let end = usize::try_from(self.end_byte.get())
            .map_err(|_| Error::Invalid("text selection end"))?;
        let text = std::str::from_utf8(bytes).map_err(|_| Error::Invalid("anchor text utf-8"))?;
        if end > bytes.len() || !text.is_char_boundary(start) || !text.is_char_boundary(end) {
            return Err(Error::Invalid("text selection boundary"));
        }
        if ReviewDigest::sha256(&bytes[start..end]) != self.selected_sha256 {
            return Err(Error::Invalid("text selection digest"));
        }
        Ok(())
    }
}

pub(crate) fn valid_label(value: &str, max: usize) -> bool {
    !value.is_empty() && value.len() <= max && !value.chars().any(|c| c.is_control())
}

pub(crate) fn valid_repo_path(value: &str) -> bool {
    valid_label(value, 4096)
        && !value.starts_with('/')
        && !value.contains('\\')
        && value
            .split('/')
            .all(|component| !component.is_empty() && !matches!(component, "." | ".."))
}
