use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use sha2::{Digest as _, Sha256};
use std::{fmt, str::FromStr};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("invalid {kind}: expected {bytes} bytes or canonical lowercase hex")]
pub struct InvalidReviewId {
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
            pub fn from_slice(bytes: &[u8]) -> Result<Self, InvalidReviewId> {
                bytes.try_into().map(Self).map_err(|_| InvalidReviewId {
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
            type Err = InvalidReviewId;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let fail = || InvalidReviewId {
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

id!(ReviewThreadId, 16);
id!(CommentId, 16);
id!(ReviewActionId, 16);
id!(ReviewWorkRequestId, 16);
id!(AnchorId, 32);
id!(ArtifactId, 32);
id!(CommentRevisionId, 32);
id!(ReviewEventId, 32);
id!(ThreadSnapshotId, 32);
id!(ReviewRootId, 32);
id!(ReviewWorkRequestVersionId, 32);
id!(ReviewDigest, 32);
id!(ReviewObjectId, 32);
id!(ReviewCatalogKey, 32);

impl ReviewDigest {
    pub fn sha256(bytes: &[u8]) -> Self {
        Self::new(Sha256::digest(bytes).into())
    }
}

impl ReviewCatalogKey {
    pub fn derive(namespace: &str, value: &[u8]) -> crate::Result<Self> {
        if namespace.is_empty()
            || namespace.len() > 32
            || !namespace.bytes().all(|byte| {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"-_".contains(&byte)
            })
            || value.is_empty()
            || value.len() > 1024
        {
            return Err(crate::Error::Invalid("review catalog key"));
        }
        let mut hash = Sha256::new_with_prefix(b"jj-store/review-catalog-key/v1\0");
        hash.update((namespace.len() as u64).to_be_bytes());
        hash.update(namespace.as_bytes());
        hash.update((value.len() as u64).to_be_bytes());
        hash.update(value);
        Ok(Self::new(hash.finalize().into()))
    }
}
