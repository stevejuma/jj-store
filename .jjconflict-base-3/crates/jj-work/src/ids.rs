use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use std::{fmt, str::FromStr};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("invalid {kind}: expected {bytes} bytes or canonical lowercase hex")]
pub struct InvalidWorkId {
    pub kind: &'static str,
    pub bytes: usize,
}

macro_rules! id {
    ($name:ident, $size:literal) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name([u8; $size]);
        impl $name {
            pub const fn new(bytes: [u8; $size]) -> Self {
                Self(bytes)
            }
            pub const fn as_bytes(&self) -> &[u8; $size] {
                &self.0
            }
            pub fn from_slice(bytes: &[u8]) -> Result<Self, InvalidWorkId> {
                bytes.try_into().map(Self).map_err(|_| InvalidWorkId {
                    kind: stringify!($name),
                    bytes: $size,
                })
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&hex::encode(self.0))
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({self})", stringify!($name))
            }
        }
        impl FromStr for $name {
            type Err = InvalidWorkId;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let fail = || InvalidWorkId {
                    kind: stringify!($name),
                    bytes: $size,
                };
                if value.len() != $size * 2
                    || !value
                        .bytes()
                        .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
                {
                    return Err(fail());
                }
                let mut bytes = [0; $size];
                hex::decode_to_slice(value, &mut bytes).map_err(|_| fail())?;
                Ok(Self(bytes))
            }
        }
        impl schemars::JsonSchema for $name {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                stringify!($name).into()
            }
            fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({
                    "type": "string",
                    "pattern": format!("^[0-9a-f]{{{}}}$", $size * 2),
                    "minLength": $size * 2,
                    "maxLength": $size * 2
                })
            }
        }
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                String::deserialize(deserializer)?
                    .parse()
                    .map_err(D::Error::custom)
            }
        }
    };
}

id!(ProjectId, 16);
id!(TaskId, 16);
id!(WorkCommentId, 16);
