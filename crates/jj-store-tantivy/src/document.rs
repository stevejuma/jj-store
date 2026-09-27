use crate::*;
use std::collections::BTreeSet;
use tantivy::schema::{Field, STORED, STRING, Schema, TEXT, TantivyDocument};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DocumentKey {
    Operation { id: m::OperationId },
    Commit { id: m::CommitId },
    File { scope: String, path: String },
}
impl DocumentKey {
    pub fn exact(&self) -> Result<String> {
        Ok(match self {
            Self::Operation { id } => format!("op/{id}"),
            Self::Commit { id } => format!("commit/{id}"),
            Self::File { scope, path } => {
                if !valid_scope(scope)
                    || path.is_empty()
                    || path.len() > 4096
                    || path.contains(['\0', '\\'])
                    || path
                        .split('/')
                        .any(|p| p.is_empty() || p == "." || p == "..")
                {
                    return Err(Error::Invalid);
                }
                format!("file/{scope}/{path}")
            }
        })
    }
}
fn valid_scope(scope: &str) -> bool {
    !scope.is_empty()
        && scope.len() <= 128
        && scope
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_:.".contains(&b))
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeState {
    Ready,
    Conflicted,
    Missing,
    AmbiguousHeads,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopeSnapshot {
    pub state: ScopeState,
    pub operation: Option<m::OperationId>,
    pub commits: BTreeSet<m::CommitId>,
}
impl ScopeSnapshot {
    pub(crate) fn validate(&self) -> Result<()> {
        if (self.state == ScopeState::Ready
            && (self.commits.len() != 1 || self.operation.is_none()))
            || self.commits.len() > 1024
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
/// Owned extraction output. Native identity fields are stored exactly; the body
/// is indexed but never stored in query hits. Fetch source via an authorized ID.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Document {
    pub key: DocumentKey,
    pub title: String,
    pub body: String,
    pub actor: Option<String>,
    pub author: String,
    pub workspace: Option<String>,
    pub operation: Option<m::OperationId>,
    pub commit: Option<m::CommitId>,
    pub change: Option<m::ChangeId>,
    pub file: Option<m::FileId>,
    pub parents: Vec<m::CommitId>,
    pub predecessors: Vec<m::CommitId>,
    pub attributes: BTreeMap<String, String>,
    pub language: Option<String>,
    pub has_conflict: bool,
    pub timestamp_millis: i64,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Update {
    pub upsert: Vec<Document>,
    pub delete: BTreeSet<DocumentKey>,
    pub replace_scopes: BTreeSet<String>,
    pub scopes: BTreeMap<String, ScopeSnapshot>,
}
impl Update {
    pub(crate) fn validate(&self, limits: Limits) -> Result<()> {
        if self.upsert.len().saturating_add(self.delete.len()) > limits.documents
            || self.scopes.len() > 128
        {
            return Err(Error::Limit);
        }
        for (name, scope) in &self.scopes {
            if !valid_scope(name) {
                return Err(Error::Invalid);
            }
            scope.validate()?;
        }
        for name in &self.replace_scopes {
            if !valid_scope(name) {
                return Err(Error::Invalid);
            }
        }
        let mut keys = BTreeSet::new();
        let mut bytes = 0usize;
        for document in &self.upsert {
            let key = document.key.exact()?;
            if !keys.insert(key) || self.delete.contains(&document.key) {
                return Err(Error::Invalid);
            }
            let size = bounded_json(document, limits.document_bytes)?.len();
            bytes = bytes.checked_add(size).ok_or(Error::Limit)?;
            if size > limits.document_bytes || bytes > limits.update_bytes {
                return Err(Error::Limit);
            }
            match &document.key {
                DocumentKey::Operation { id } if document.operation != Some(*id) => {
                    return Err(Error::Invalid);
                }
                DocumentKey::Commit { id } if document.commit != Some(*id) => {
                    return Err(Error::Invalid);
                }
                DocumentKey::File { scope, .. } => {
                    let scope = self.scopes.get(scope).ok_or(Error::Invalid)?;
                    if scope.state != ScopeState::Ready
                        || scope.operation != document.operation
                        || !document.commit.is_some_and(|c| scope.commits.contains(&c))
                        || document.file.is_none()
                    {
                        return Err(Error::Invalid);
                    }
                }
                _ => {}
            }
        }
        for key in &self.delete {
            key.exact()?;
        }
        bounded_json(&self.scopes, 256 * 1024)?;
        bounded_json(self, limits.update_bytes)?;
        Ok(())
    }
}
#[derive(Clone)]
pub(crate) struct Fields {
    pub schema: Schema,
    pub key: Field,
    pub text: Field,
    pub title: Field,
    pub path: Field,
    pub body: Field,
    pub scope: Field,
    pub id: Field,
    pub stored: Field,
}
impl Fields {
    pub fn new() -> Self {
        let mut b = Schema::builder();
        let key = b.add_text_field("key", STRING | STORED);
        let text = b.add_text_field("metadata", TEXT);
        let title = b.add_text_field("title", TEXT);
        let path = b.add_text_field("path_tokens", TEXT);
        let body = b.add_text_field("body", TEXT);
        let scope = b.add_text_field("scope_id", STRING);
        let id = b.add_text_field("native_id", STRING);
        let stored = b.add_text_field("record", STORED);
        Self {
            schema: b.build(),
            key,
            text,
            title,
            path,
            body,
            scope,
            id,
            stored,
        }
    }
    pub fn document(&self, value: &Document) -> Result<TantivyDocument> {
        let mut doc = TantivyDocument::new();
        doc.add_text(self.key, value.key.exact()?);
        for id in [
            value.operation.map(|id| id.to_string()),
            value.commit.map(|id| id.to_string()),
            value.change.map(|id| id.to_string()),
            value.file.map(|id| id.to_string()),
        ]
        .into_iter()
        .flatten()
        {
            doc.add_text(self.id, id);
        }
        doc.add_text(self.title, &value.title);
        doc.add_text(self.body, &value.body);
        doc.add_text(self.text, &value.author);
        if let Some(actor) = &value.actor {
            doc.add_text(self.text, actor);
        }
        if let Some(workspace) = &value.workspace {
            doc.add_text(self.text, workspace);
        }
        for (key, val) in &value.attributes {
            doc.add_text(self.text, format!("{key} {val}"));
        }
        if let DocumentKey::File { path, scope } = &value.key {
            doc.add_text(self.path, path);
            doc.add_text(self.scope, scope);
        }
        let mut stored = value.clone();
        stored.body.clear();
        doc.add_text(
            self.stored,
            serde_json::to_string(&stored).map_err(|_| Error::Invalid)?,
        );
        Ok(doc)
    }
}
