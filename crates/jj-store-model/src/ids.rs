use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("invalid {kind}: expected {bytes} bytes or canonical lowercase hex")]
pub struct InvalidId {
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
            pub fn from_slice(bytes: &[u8]) -> Result<Self, InvalidId> {
                bytes.try_into().map(Self).map_err(|_| InvalidId {
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
            type Err = InvalidId;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let fail = || InvalidId {
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
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(&self.to_string())
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                String::deserialize(d)?.parse().map_err(D::Error::custom)
            }
        }
    };
}

id!(CommitId, 64);
id!(TreeId, 64);
id!(FileId, 64);
id!(SymlinkId, 64);
id!(CopyId, 64);
id!(ViewId, 64);
id!(OperationId, 64);
id!(ChangeId, 16);
id!(RepositoryId, 16);
id!(RequestId, 16);
id!(PublicationId, 16);
id!(EnvelopeDigest, 32);
id!(RecordDigest, 32);
id!(IntentDigest, 32);
id!(AttemptId, 16);
id!(EpochId, 16);
id!(ReplicaId, 16);
id!(WriterEpoch, 16);
id!(OriginDigest, 32);
id!(PrincipalId, 16);
id!(TokenId, 16);
id!(ConflictFingerprint, 64);
id!(ConflictUploadId, 32);

/// A copy placeholder is not a content-addressed object or dependency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "type",
    content = "id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum CopyReference {
    Placeholder,
    Object(CopyId),
}

/// Publication order is distinct from native operation ancestry.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(transparent)]
pub struct WalPosition(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ObjectKind {
    Commit = 1,
    Tree = 2,
    File = 3,
    Symlink = 4,
    Copy = 5,
    View = 6,
    Operation = 7,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
)]
#[serde(
    tag = "kind",
    content = "id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ObjectKey {
    Commit(CommitId),
    Tree(TreeId),
    File(FileId),
    Symlink(SymlinkId),
    Copy(CopyId),
    View(ViewId),
    Operation(OperationId),
}
impl ObjectKey {
    pub fn kind(self) -> ObjectKind {
        match self {
            Self::Commit(_) => ObjectKind::Commit,
            Self::Tree(_) => ObjectKind::Tree,
            Self::File(_) => ObjectKind::File,
            Self::Symlink(_) => ObjectKind::Symlink,
            Self::Copy(_) => ObjectKind::Copy,
            Self::View(_) => ObjectKind::View,
            Self::Operation(_) => ObjectKind::Operation,
        }
    }
    pub fn as_bytes(&self) -> &[u8; 64] {
        match self {
            Self::Commit(id) => id.as_bytes(),
            Self::Tree(id) => id.as_bytes(),
            Self::File(id) => id.as_bytes(),
            Self::Symlink(id) => id.as_bytes(),
            Self::Copy(id) => id.as_bytes(),
            Self::View(id) => id.as_bytes(),
            Self::Operation(id) => id.as_bytes(),
        }
    }
    pub fn from_parts(kind: u8, bytes: [u8; 64]) -> Option<Self> {
        Some(match kind {
            1 => Self::Commit(CommitId::new(bytes)),
            2 => Self::Tree(TreeId::new(bytes)),
            3 => Self::File(FileId::new(bytes)),
            4 => Self::Symlink(SymlinkId::new(bytes)),
            5 => Self::Copy(CopyId::new(bytes)),
            6 => Self::View(ViewId::new(bytes)),
            7 => Self::Operation(OperationId::new(bytes)),
            _ => return None,
        })
    }
    /// Stable repository-local key; no caller-provided path components.
    pub fn storage_path(self, repository: RepositoryId) -> String {
        format!(
            "repos/{repository}/objects/v1/{}/{}/{}",
            self.kind() as u8,
            hex::encode(&self.as_bytes()[..2]),
            hex::encode(self.as_bytes())
        )
    }
}
