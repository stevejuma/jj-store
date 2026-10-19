use super::*;
use std::{
    fs::{self, File, OpenOptions},
    path::Path,
};
use tantivy::{
    Index, IndexReader, IndexWriter, Term,
    collector::TopDocs,
    query::TermQuery,
    schema::{
        FAST, Field, IndexRecordOption, STORED, STRING, Schema, TEXT, TantivyDocument, Value as _,
    },
};

pub(super) struct Fields {
    pub key: Field,
    pub kind: Field,
    pub body: Field,
    pub content: Field,
    pub path: Field,
    pub description: Field,
    pub author: Field,
    pub commit: Field,
    pub time: Field,
    pub record: Field,
}
fn schema() -> (Schema, Fields) {
    let mut s = Schema::builder();
    let fields = Fields {
        key: s.add_text_field("key", STRING | STORED),
        kind: s.add_text_field("kind", STRING),
        body: s.add_text_field("body", TEXT | STORED),
        content: s.add_text_field("content", STRING | STORED),
        path: s.add_text_field("path", STRING),
        description: s.add_text_field("description", TEXT),
        author: s.add_text_field("author", STRING),
        commit: s.add_text_field("commit", STRING),
        time: s.add_i64_field("time", FAST | tantivy::schema::INDEXED),
        record: s.add_text_field("record", STORED),
    };
    (s.build(), fields)
}
pub struct Engine {
    pub(super) index: Index,
    pub(super) reader: IndexReader,
    pub(super) fields: Fields,
    pub(super) state: State,
    _owner: File,
}
impl Engine {
    /// The host selects this private directory, never a renderer path.
    pub fn open(root: &Path, namespace: &str) -> Result<Self, String> {
        fs::create_dir_all(root).map_err(error)?;
        let owner = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(root.join("owner.lock"))
            .map_err(error)?;
        owner.try_lock().map_err(|_| "History index is busy")?;
        let path = root.join("index");
        fs::create_dir_all(&path).map_err(error)?;
        let (definition, fields) = schema();
        let index = match Index::open_in_dir(&path) {
            Ok(index) if index.schema() == definition => index,
            _ => {
                // Only disposable bytes below the admitted index root are retired.
                fs::remove_dir_all(&path).map_err(error)?;
                fs::create_dir(&path).map_err(error)?;
                Index::create_in_dir(&path, definition).map_err(error)?
            }
        };
        let payload = index.load_metas().map_err(error)?.payload;
        let recovered = payload
            .as_deref()
            .and_then(|text| serde_json::from_str::<State>(text).ok())
            .filter(|state| state.namespace == namespace);
        let index = if payload.is_some() && recovered.is_none() {
            drop(index);
            fs::remove_dir_all(&path).map_err(error)?;
            fs::create_dir(&path).map_err(error)?;
            Index::create_in_dir(&path, schema().0).map_err(error)?
        } else {
            index
        };
        let state = recovered.unwrap_or_else(|| State {
            namespace: namespace.into(),
            ..State::default()
        });
        let reader = index.reader().map_err(error)?;
        Ok(Self {
            index,
            reader,
            fields,
            state,
            _owner: owner,
        })
    }
    pub fn status(&self) -> Status {
        Status {
            generation: self.state.generation,
            indexed: self.state.indexed,
            omitted: self.state.omitted,
            complete: self.state.pending.is_empty(),
        }
    }
    pub(super) fn term(&self, field: Field, value: &str) -> Box<dyn tantivy::query::Query> {
        Box::new(TermQuery::new(
            Term::from_field_text(field, value),
            IndexRecordOption::Basic,
        ))
    }
    pub(super) fn lookup(&self, key: &str) -> Result<Option<TantivyDocument>, String> {
        let searcher = self.reader.searcher();
        searcher
            .search(
                &self.term(self.fields.key, key),
                &TopDocs::with_limit(1).order_by_score(),
            )
            .map_err(error)?
            .first()
            .map(|(_, address)| searcher.doc(*address).map_err(error))
            .transpose()
    }
    pub(super) fn stored<'a>(&self, doc: &'a TantivyDocument, field: Field) -> &'a str {
        doc.get_first(field)
            .and_then(|value| value.as_str())
            .unwrap_or("")
    }
    fn write_record(
        &self,
        writer: &mut IndexWriter,
        key: &str,
        kind: &str,
        record: &Record,
    ) -> Result<(), String> {
        let mut doc = TantivyDocument::default();
        doc.add_text(self.fields.key, key);
        doc.add_text(self.fields.kind, kind);
        doc.add_text(
            self.fields.content,
            record.content_id.as_deref().unwrap_or(""),
        );
        doc.add_text(
            self.fields.path,
            record.path.as_deref().unwrap_or("").to_lowercase(),
        );
        doc.add_text(
            self.fields.description,
            &record.revision.checkpoint.description,
        );
        doc.add_text(
            self.fields.author,
            format!("{} {}", record.revision.author, record.revision.email).to_lowercase(),
        );
        doc.add_text(self.fields.commit, &record.revision.checkpoint.commit);
        doc.add_i64(
            self.fields.time,
            record.revision.checkpoint.timestamp_millis,
        );
        doc.add_text(
            self.fields.record,
            serde_json::to_string(record).map_err(error)?,
        );
        writer.delete_term(Term::from_field_text(self.fields.key, key));
        writer.add_document(doc).map_err(error)?;
        Ok(())
    }
    /// One changed-file page. Capture never waits for this disposable publication.
    pub fn step(&mut self, source: &mut impl Source) -> Result<Status, String> {
        let previous = self.state.clone();
        let result = self.step_inner(source);
        if result.is_err() {
            self.state = previous;
        }
        result
    }
    fn step_inner(&mut self, source: &mut impl Source) -> Result<Status, String> {
        let head = source.head()?;
        if head != self.state.anchor {
            self.state.anchor = head.clone();
            if let Some(commit) = head {
                self.state.pending.push(Work {
                    commit,
                    after: None,
                });
            }
        }
        let Some(work) = self.state.pending.pop() else {
            return Ok(self.status());
        };
        let mut writer = self
            .index
            .writer_with_num_threads::<TantivyDocument>(1, 16 * 1024 * 1024)
            .map_err(error)?;
        if self
            .lookup(&format!("checkpoint/{}", work.commit))?
            .is_none()
        {
            let revision = source.revision(&work.commit)?;
            let page = source.files(&work.commit, work.after.as_deref())?;
            let mut seen = std::collections::BTreeSet::new();
            for file in page.files {
                if let (Some(id), Some(text)) = (&file.content_id, &file.text)
                    && seen.insert(id.clone())
                    && self.lookup(&format!("body/{id}"))?.is_none()
                {
                    let mut doc = TantivyDocument::default();
                    doc.add_text(self.fields.key, format!("body/{id}"));
                    doc.add_text(self.fields.kind, "body");
                    doc.add_text(self.fields.content, id);
                    doc.add_text(self.fields.body, text);
                    writer.add_document(doc).map_err(error)?;
                }
                let unavailable = file.text.is_none() && !file.deleted;
                if unavailable {
                    self.state.omitted += 1;
                }
                let key = format!("event/{}/{}", work.commit, file.path);
                self.write_record(
                    &mut writer,
                    &key,
                    "event",
                    &Record {
                        revision: revision.revision.clone(),
                        parents: revision.parents.clone(),
                        path: Some(file.path),
                        content_id: file.content_id,
                        deleted: file.deleted,
                        previous: file.previous,
                        unavailable,
                    },
                )?;
            }
            if let Some(after) = page.next {
                self.state.pending.push(Work {
                    commit: work.commit,
                    after: Some(after),
                });
            } else {
                self.write_record(
                    &mut writer,
                    &format!("checkpoint/{}", work.commit),
                    "checkpoint",
                    &Record {
                        revision: revision.revision,
                        parents: revision.parents.clone(),
                        path: None,
                        content_id: None,
                        deleted: false,
                        previous: None,
                        unavailable: false,
                    },
                )?;
                self.state.indexed += 1;
                // Every parent is visited independently; merges never truncate backfill.
                self.state
                    .pending
                    .extend(revision.parents.into_iter().map(|commit| Work {
                        commit,
                        after: None,
                    }));
            }
        }
        self.state.generation += 1;
        let mut prepared = writer.prepare_commit().map_err(error)?;
        prepared.set_payload(&serde_json::to_string(&self.state).map_err(error)?);
        prepared.commit().map_err(error)?;
        self.reader.reload().map_err(error)?;
        Ok(self.status())
    }
}
