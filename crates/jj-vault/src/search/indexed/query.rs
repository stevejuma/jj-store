use super::*;
use sha2::{Digest as _, Sha256};
use std::{
    ops::Bound,
    time::{Duration, Instant},
};
use tantivy::{
    Order, Term,
    collector::TopDocs,
    query::{
        AllQuery, BooleanQuery, Occur, Query, QueryParser, RangeQuery, RegexQuery, TermSetQuery,
    },
    schema::TantivyDocument,
};

impl Engine {
    fn expression(
        &self,
        expression: &Expression,
        depth: usize,
        nodes: &mut usize,
        case_sensitive: bool,
    ) -> Result<Box<dyn Query>, String> {
        *nodes += 1;
        if depth > 16 || *nodes > 128 {
            return Err("History query is too complex".into());
        }
        Ok(match expression {
            Expression::And { clauses } => Box::new(BooleanQuery::intersection(
                clauses
                    .iter()
                    .map(|c| self.expression(c, depth + 1, nodes, case_sensitive))
                    .collect::<Result<_, _>>()?,
            )),
            Expression::Or { clauses } => Box::new(BooleanQuery::union(
                clauses
                    .iter()
                    .map(|c| self.expression(c, depth + 1, nodes, case_sensitive))
                    .collect::<Result<_, _>>()?,
            )),
            Expression::Not { .. } if case_sensitive => Box::new(AllQuery),
            Expression::Not { operand } => Box::new(BooleanQuery::new(vec![
                (Occur::Must, Box::new(AllQuery)),
                (
                    Occur::MustNot,
                    self.expression(operand, depth + 1, nodes, case_sensitive)?,
                ),
            ])),
            Expression::Term { field, value } => {
                if value.len() > 4096 || value.is_empty() {
                    return Err("Invalid history query value".into());
                }
                let field = field.as_deref();
                match field {
                    Some("after" | "before") => {
                        let time = value
                            .parse::<i64>()
                            .map_err(|_| "Invalid checkpoint date")?;
                        let term = Term::from_field_i64(self.fields.time, time);
                        let (lower, upper) = if field == Some("after") {
                            (Bound::Included(term), Bound::Unbounded)
                        } else {
                            (Bound::Unbounded, Bound::Included(term))
                        };
                        Box::new(RangeQuery::new(lower, upper))
                    }
                    Some("path" | "file" | "author" | "commit") => {
                        let target = match field {
                            Some("author") => self.fields.author,
                            Some("commit") => self.fields.commit,
                            _ => self.fields.path,
                        };
                        let value = regex::escape(&value.to_lowercase());
                        let pattern = if field == Some("file") {
                            format!("(.*/)?[^/]*{value}[^/]*")
                        } else {
                            format!(".*{value}.*")
                        };
                        Box::new(RegexQuery::from_pattern(&pattern, target).map_err(error)?)
                    }
                    None | Some("content") => {
                        let escaped =
                            format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""));
                        let parser = QueryParser::for_index(&self.index, vec![self.fields.body]);
                        let bodies = BooleanQuery::intersection(vec![
                            self.term(self.fields.kind, "body"),
                            parser.parse_query(&escaped).map_err(error)?,
                        ]);
                        let searcher = self.reader.searcher();
                        let found = searcher
                            .search(&bodies, &TopDocs::with_limit(10_001).order_by_score())
                            .map_err(error)?;
                        if found.len() > 10_000 {
                            return Err(
                                "History query exceeds its candidate budget; narrow the query"
                                    .into(),
                            );
                        }
                        let terms = found
                            .into_iter()
                            .map(|(_, address)| {
                                let doc: TantivyDocument = searcher.doc(address).map_err(error)?;
                                Ok(Term::from_field_text(
                                    self.fields.content,
                                    self.stored(&doc, self.fields.content),
                                ))
                            })
                            .collect::<Result<Vec<_>, String>>()?;
                        let content: Box<dyn Query> = Box::new(TermSetQuery::new(terms));
                        if field == Some("content") {
                            content
                        } else {
                            let path = RegexQuery::from_pattern(
                                &format!(".*{}.*", regex::escape(&value.to_lowercase())),
                                self.fields.path,
                            )
                            .map_err(error)?;
                            let title =
                                QueryParser::for_index(&self.index, vec![self.fields.description])
                                    .parse_query(&escaped)
                                    .map_err(error)?;
                            Box::new(BooleanQuery::union(vec![
                                content,
                                Box::new(path),
                                Box::new(BooleanQuery::intersection(vec![
                                    self.term(self.fields.kind, "checkpoint"),
                                    title,
                                ])),
                            ]))
                        }
                    }
                    _ => return Err("Unsupported history filter".into()),
                }
            }
        })
    }

    pub fn search(&self, source: &mut impl Source, request: &Request) -> Result<Page, String> {
        if request.limit == 0 || request.limit > 100 {
            return Err("Invalid history page limit".into());
        }
        let query_id = super::super::lower_hex(Sha256::digest(
            serde_json::to_vec(&(&request.expression, request.case_sensitive)).map_err(error)?,
        ));
        let cursor: Option<Cursor> = request
            .cursor
            .as_ref()
            .map(|text| {
                if text.len() > 512 {
                    return Err("Invalid history cursor".into());
                }
                serde_json::from_str(text).map_err(error)
            })
            .transpose()?;
        if cursor
            .as_ref()
            .is_some_and(|cursor| cursor.query != query_id || cursor.offset > 100_000)
        {
            return Err("Invalid history cursor".into());
        }
        let reset = cursor
            .as_ref()
            .is_some_and(|cursor| cursor.generation != self.state.generation);
        let offset = if reset {
            0
        } else {
            cursor.map_or(0, |cursor| cursor.offset)
        };
        let filter = self.expression(&request.expression, 0, &mut 0, request.case_sensitive)?;
        let kinds = BooleanQuery::union(vec![
            self.term(self.fields.kind, "event"),
            self.term(self.fields.kind, "checkpoint"),
        ]);
        let query = BooleanQuery::intersection(vec![Box::new(kinds), filter]);
        let searcher = self.reader.searcher();
        let candidates = searcher
            .search(
                &query,
                &TopDocs::with_limit(101)
                    .and_offset(offset)
                    .order_by_fast_field::<i64>("time", Order::Desc),
            )
            .map_err(error)?;
        let more = candidates.len() > 100;
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut hits = vec![];
        let mut consumed = 0;
        for (_, address) in candidates.iter().take(100) {
            if Instant::now() > deadline {
                break;
            }
            consumed += 1;
            let doc: TantivyDocument = searcher.doc(*address).map_err(error)?;
            let record: Record =
                serde_json::from_str(self.stored(&doc, self.fields.record)).map_err(error)?;
            if self
                .lookup(&format!("checkpoint/{}", record.revision.checkpoint.commit))?
                .is_none()
            {
                continue;
            }
            // Disposable rows do not establish retained source membership.
            source.revision(&record.revision.checkpoint.commit)?;
            let text = if let Some(id) = &record.content_id {
                self.lookup(&format!("body/{id}"))?
                    .map(|body| self.stored(&body, self.fields.body).to_owned())
                    .unwrap_or_default()
            } else {
                String::new()
            };
            if request.case_sensitive && !exact(&request.expression, &record, &text) {
                continue;
            }
            let (line, excerpt) = excerpt(&record, &text, &request.expression);
            hits.push(Hit {
                record,
                excerpt,
                line,
            });
            if hits.len() == request.limit {
                break;
            }
        }
        let next_offset = offset + consumed;
        let remaining = consumed < candidates.len().min(100) || more;
        let next = if remaining {
            Some(
                serde_json::to_string(&Cursor {
                    generation: self.state.generation,
                    offset: next_offset,
                    query: query_id,
                })
                .map_err(error)?,
            )
        } else {
            None
        };
        let mut status = self.status();
        if source.head()? != self.state.anchor {
            status.complete = false;
        }
        Ok(Page {
            status,
            hits,
            next,
            reset,
        })
    }
}
fn exact(expr: &Expression, record: &Record, body: &str) -> bool {
    match expr {
        Expression::And { clauses } => clauses.iter().all(|c| exact(c, record, body)),
        Expression::Or { clauses } => clauses.iter().any(|c| exact(c, record, body)),
        Expression::Not { operand } => !exact(operand, record, body),
        Expression::Term { field, value } => match field.as_deref() {
            Some("after" | "before") => true,
            Some("content") => body.contains(value),
            Some("path") => record.path.as_deref().unwrap_or("").contains(value),
            Some("file") => record
                .path
                .as_deref()
                .unwrap_or("")
                .rsplit('/')
                .next()
                .unwrap_or("")
                .contains(value),
            Some("author") => {
                format!("{} {}", record.revision.author, record.revision.email).contains(value)
            }
            Some("commit") => record.revision.checkpoint.commit.contains(value),
            _ => {
                body.contains(value)
                    || record.path.as_deref().unwrap_or("").contains(value)
                    || record.revision.checkpoint.description.contains(value)
            }
        },
    }
}
fn term(expr: &Expression) -> Option<&str> {
    match expr {
        Expression::Term { field, value }
            if field.is_none() || field.as_deref() == Some("content") =>
        {
            Some(value)
        }
        Expression::And { clauses } | Expression::Or { clauses } => clauses.iter().find_map(term),
        _ => None,
    }
}
fn excerpt(record: &Record, text: &str, expression: &Expression) -> (Option<u32>, String) {
    if record.path.is_none() {
        return (
            None,
            record
                .revision
                .checkpoint
                .description
                .lines()
                .next()
                .unwrap_or("")
                .chars()
                .take(240)
                .collect(),
        );
    }
    if text.is_empty() {
        return (
            None,
            if record.deleted {
                "File deleted in this checkpoint"
            } else {
                "Path indexed; contents unavailable for text search"
            }
            .into(),
        );
    }
    let needle = term(expression).unwrap_or("").to_lowercase();
    let (line, text) = text
        .lines()
        .enumerate()
        .find(|(_, text)| text.to_lowercase().contains(&needle))
        .unwrap_or((0, ""));
    (Some(line as u32 + 1), text.chars().take(240).collect())
}
