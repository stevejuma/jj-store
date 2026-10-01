use jj_store_model::{IntentDigest, PrincipalId, RepositoryId, RequestId};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt, str::FromStr};

use super::{Error, Result};

/// Git identities remain distinct from fixed-width native JJ identities.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GitOid(pub(crate) gix::ObjectId);
impl GitOid {
    pub fn from_git(value: gix::ObjectId) -> Result<Self> {
        if value.kind() != gix::hash::Kind::Sha1 || value.is_null() {
            return Err(Error::Invalid("Git SHA-1 identity"));
        }
        Ok(Self(value))
    }
    pub fn as_git(self) -> gix::ObjectId {
        self.0
    }
}
impl fmt::Display for GitOid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Debug for GitOid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GitOid({self})")
    }
}
impl FromStr for GitOid {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self> {
        if value.len() != 40
            || value
                .bytes()
                .any(|v| !v.is_ascii_digit() && !(b'a'..=b'f').contains(&v))
        {
            return Err(Error::Invalid("canonical Git SHA-1 identity"));
        }
        Self::from_git(gix::ObjectId::from_hex(value.as_bytes()).map_err(Error::other)?)
    }
}
impl Serialize for GitOid {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for GitOid {
    fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> std::result::Result<Self, D::Error> {
        String::deserialize(decoder)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
impl schemars::JsonSchema for GitOid {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GitSha1Id".into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({"type":"string","pattern":"^[0-9a-f]{40}$","minLength":40,"maxLength":40})
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    Work,
    Agent,
    Review,
    Thread,
    Reaction,
}
impl Domain {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Work => "work",
            Self::Agent => "agent",
            Self::Review => "review",
            Self::Thread => "thread",
            Self::Reaction => "reaction",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    pub object: GitOid,
    pub version: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetainedKind {
    Blob,
    Tree,
    Commit,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub repository: RepositoryId,
    pub actor: PrincipalId,
    pub request: RequestId,
    pub intent: IntentDigest,
    pub sequence: u64,
    pub domain: Domain,
    pub selection: Selection,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub selections: BTreeMap<Domain, Selection>,
    pub result: Vec<u8>,
}
impl Receipt {
    pub(crate) fn key(actor: PrincipalId, request: RequestId) -> String {
        format!("{actor}:{request}")
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RootV1 {
    pub version: u32,
    pub repository: RepositoryId,
    pub sequence: u64,
    pub previous: Option<GitOid>,
    pub domains: BTreeMap<Domain, Selection>,
    pub receipts: BTreeMap<String, Receipt>,
    pub retained: BTreeMap<GitOid, RetainedKind>,
}
#[derive(Clone, Debug)]
pub struct Snapshot {
    pub(crate) commit: GitOid,
    pub(crate) root: RootV1,
}
impl Snapshot {
    pub fn format_version(&self) -> u32 {
        self.root.version
    }
    pub fn commit(&self) -> GitOid {
        self.commit
    }
    pub fn repository(&self) -> RepositoryId {
        self.root.repository
    }
    pub fn sequence(&self) -> u64 {
        self.root.sequence
    }
    pub fn selection(&self, domain: Domain) -> Option<&Selection> {
        self.root.domains.get(&domain)
    }
    pub fn receipt(&self, actor: PrincipalId, request: RequestId) -> Option<&Receipt> {
        self.root.receipts.get(&Receipt::key(actor, request))
    }
}
/// Validated domain services supply canonical domain bytes and their closure.
pub struct Update {
    pub repository: RepositoryId,
    pub actor: PrincipalId,
    pub request: RequestId,
    pub intent: IntentDigest,
    pub domain: Domain,
    pub expected: Option<Selection>,
    pub bytes: Vec<u8>,
    pub result: Vec<u8>,
    pub retained: BTreeMap<GitOid, RetainedKind>,
}
#[derive(Debug)]
pub struct Candidate {
    pub(crate) expected: Option<GitOid>,
    pub(crate) snapshot: Snapshot,
    pub(crate) receipt: Receipt,
}
impl Candidate {
    pub fn commit(&self) -> GitOid {
        self.snapshot.commit
    }
}
#[derive(Debug)]
pub enum Preparation {
    Ready(Box<Candidate>),
    Committed(Receipt),
}
#[derive(Debug, PartialEq, Eq)]
pub enum PublishOutcome {
    Committed(Receipt),
    Conflict,
    Unknown,
}

/// One expected-old-ref publication selects every named domain together.
pub struct DomainUpdate {
    pub expected: Option<Selection>,
    pub bytes: Vec<u8>,
}
pub struct BatchUpdate {
    pub repository: RepositoryId,
    pub actor: PrincipalId,
    pub request: RequestId,
    pub intent: IntentDigest,
    pub domains: BTreeMap<Domain, DomainUpdate>,
    pub result: Vec<u8>,
    pub retained: BTreeMap<GitOid, RetainedKind>,
}
