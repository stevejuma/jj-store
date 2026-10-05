use crate::{Error, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use sha2::{Digest as _, Sha256};
use std::{fmt, str::FromStr};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("invalid {kind}: expected {bytes} bytes or canonical lowercase hex")]
pub struct InvalidAgentId {
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
            pub fn from_slice(bytes: &[u8]) -> std::result::Result<Self, InvalidAgentId> {
                bytes.try_into().map(Self).map_err(|_| InvalidAgentId {
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
            type Err = InvalidAgentId;
            fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
                let fail = || InvalidAgentId {
                    kind: stringify!($name),
                    bytes: $size,
                };
                if value.len() != $size * 2
                    || value
                        .bytes()
                        .any(|b| !b.is_ascii_digit() && !(b'a'..=b'f').contains(&b))
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
            fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
                String::deserialize(deserializer)?.parse().map_err(D::Error::custom)
            }
        }
    };
}

id!(AgentSessionId, 16);
id!(ExecutionAttemptId, 16);
id!(TurnId, 16);
id!(MessageId, 16);
id!(BlockId, 16);
id!(ToolCallId, 16);
id!(ToolAttemptId, 16);
id!(TerminalId, 16);
id!(ProvenanceScopeId, 16);
id!(AssociationId, 16);
id!(ObservationId, 16);
id!(AgentEventId, 16);
id!(CaptureBatchId, 16);
id!(AgentCheckpointId, 16);
id!(DeviceId, 16);
id!(SourceInstanceId, 16);
id!(AgentObjectId, 32);
id!(AgentCatalogKey, 32);

impl AgentCatalogKey {
    pub fn derive(namespace: &str, value: &[u8]) -> Result<Self> {
        if namespace.is_empty()
            || namespace.len() > 32
            || !namespace
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"-_".contains(&b))
            || value.is_empty()
            || value.len() > 1024
        {
            return Err(Error::Invalid("agent catalog key"));
        }
        let mut hash = Sha256::new_with_prefix(b"jj-store/agent-catalog-key/v1\0");
        hash.update((namespace.len() as u64).to_be_bytes());
        hash.update(namespace.as_bytes());
        hash.update((value.len() as u64).to_be_bytes());
        hash.update(value);
        Ok(Self::new(hash.finalize().into()))
    }
}
