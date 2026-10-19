//! Server-free, rebuildable checkpoint/body index. Never a restore authority.
mod disk;
mod native;
mod query;
pub use disk::Engine;
pub use jj_store_jj::vault_workspace::{IndexFile, IndexFiles, IndexRevision};
use serde::{Deserialize, Serialize};

pub trait Source {
    fn head(&mut self) -> Result<Option<String>, String>;
    fn revision(&mut self, id: &str) -> Result<IndexRevision, String>;
    fn files(&mut self, id: &str, after: Option<&str>) -> Result<IndexFiles, String>;
    fn text(&mut self, id: &str, path: &str) -> Result<Option<String>, String>;
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Expression {
    Term {
        field: Option<String>,
        value: String,
    },
    And {
        clauses: Vec<Expression>,
    },
    Or {
        clauses: Vec<Expression>,
    },
    Not {
        operand: Box<Expression>,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub expression: Expression,
    pub case_sensitive: bool,
    pub cursor: Option<String>,
    pub limit: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Status {
    pub generation: u64,
    pub indexed: u64,
    pub omitted: u64,
    pub complete: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub revision: super::HistoryRevision,
    pub parents: Vec<String>,
    pub path: Option<String>,
    pub content_id: Option<String>,
    pub deleted: bool,
    pub previous: Option<String>,
    pub unavailable: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hit {
    pub record: Record,
    pub excerpt: String,
    pub line: Option<u32>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    pub status: Status,
    pub hits: Vec<Hit>,
    pub next: Option<String>,
    /// A committed generation changed; consumers restart pagination and merge IDs.
    pub reset: bool,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct State {
    namespace: String,
    anchor: Option<String>,
    pending: Vec<Work>,
    indexed: u64,
    omitted: u64,
    generation: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Work {
    commit: String,
    after: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    generation: u64,
    offset: usize,
    query: String,
}

fn error(e: impl std::fmt::Display) -> String {
    e.to_string()
}
