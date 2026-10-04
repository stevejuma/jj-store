//! Bounded operation metadata syntax, independent of commit revsets.
use super::invalid;
use crate::query::{Error, Result};
use futures::TryStreamExt as _;
use jj_lib::{operation::Operation, repo::ReadonlyRepo};

#[derive(Clone, Debug)]
enum Term {
    Text(String, String),
    Id(String),
    Snapshot(bool),
    After(i64),
    Before(i64),
}
#[derive(Clone, Debug, Default)]
pub struct Filter {
    pub from: Option<String>,
    terms: Vec<Term>,
}
impl Filter {
    pub fn parse(query: &str) -> Result<Self> {
        if query.len() > 4096 {
            return Err(Error::Limit);
        }
        let mut tokens = Vec::new();
        let mut token = String::new();
        let mut field_end = None;
        let mut quote = None;
        let mut quoted = false;
        let mut escaped = false;
        for ch in query.chars().chain(std::iter::once(' ')) {
            if ch.is_control() {
                return Err(invalid("syntax", "Operation queries must be one line."));
            }
            if escaped {
                token.push(ch);
                escaped = false;
                continue;
            }
            if quote.is_some() && ch == '\\' {
                escaped = true;
                continue;
            }
            if quote == Some(ch) {
                quote = None;
                continue;
            }
            if quote.is_some() {
                token.push(ch);
                continue;
            }
            if ch == '"' || ch == '\'' {
                quote = Some(ch);
                quoted = true;
                continue;
            }
            if ch.is_whitespace() {
                if !token.is_empty() || quoted {
                    tokens.push((std::mem::take(&mut token), field_end.take(), quoted));
                    quoted = false;
                    if tokens.len() > 64 {
                        return Err(Error::Limit);
                    }
                }
            } else {
                if ch == ':' && field_end.is_none() && !quoted {
                    field_end = Some(token.len());
                }
                token.push(ch);
            }
        }
        if quote.is_some() || escaped {
            return Err(invalid(
                "syntax",
                "Close the quoted operation filter value.",
            ));
        }
        let mut filter = Self::default();
        let mut needs_term = true;
        for (token, field_end, quoted) in tokens {
            if field_end.is_none() && !quoted {
                if token.eq_ignore_ascii_case("AND") {
                    if needs_term {
                        return Err(invalid("syntax", "AND needs a filter on each side."));
                    }
                    needs_term = true;
                    continue;
                }
                if token.eq_ignore_ascii_case("OR")
                    || token.eq_ignore_ascii_case("NOT")
                    || token.contains(['(', ')', '|', '&', '~'])
                {
                    return Err(invalid(
                        "syntax",
                        "Operation filters combine with spaces or AND; commit revsets and OR/NOT are unavailable.",
                    ));
                }
            }
            let (field, value) = field_end.map_or(("description", token.as_str()), |end| {
                (&token[..end], &token[end + 1..])
            });
            if value.is_empty() {
                return Err(invalid("syntax", "A filter field needs a value."));
            }
            needs_term = false;
            match field {
                "from" => {
                    if filter.from.is_some() || value.len() > 256 {
                        return Err(invalid("syntax", "Use one from: operation selector."));
                    }
                    let symbol = value.trim_end_matches(['-', '+']);
                    if symbol != "@"
                        && (symbol.is_empty()
                            || symbol.len() > 128
                            || !symbol.bytes().all(|b| b.is_ascii_hexdigit()))
                    {
                        return Err(invalid(
                            "syntax",
                            "from: accepts @ or an operation ID with parent/child suffixes.",
                        ));
                    }
                    filter.from = Some(value.to_owned());
                }
                "description" | "user" | "host" | "workspace" => filter
                    .terms
                    .push(Term::Text(field.into(), value.to_lowercase())),
                "id" => {
                    if value.len() > 128 || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
                        return Err(invalid(
                            "syntax",
                            "id: requires a hexadecimal operation prefix.",
                        ));
                    }
                    filter.terms.push(Term::Id(value.to_lowercase()));
                }
                "snapshot" => filter.terms.push(Term::Snapshot(match value {
                    "true" => true,
                    "false" => false,
                    _ => return Err(invalid("syntax", "snapshot: accepts true or false.")),
                })),
                "after" | "before" => {
                    let date = chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
                        .map_err(|_| invalid("syntax", "Dates use YYYY-MM-DD in UTC."))?;
                    if date.format("%Y-%m-%d").to_string() != value {
                        return Err(invalid("syntax", "Dates use YYYY-MM-DD in UTC."));
                    }
                    let millis = date
                        .and_hms_opt(0, 0, 0)
                        .ok_or(Error::Invalid)?
                        .and_utc()
                        .timestamp_millis();
                    filter.terms.push(if field == "after" {
                        Term::After(millis)
                    } else {
                        Term::Before(millis)
                    });
                }
                _ => {
                    return Err(invalid(
                        "syntax",
                        "Unknown operation field. Use description, user, host, workspace, id, after, before, snapshot or from.",
                    ));
                }
            }
        }
        if needs_term && (!filter.terms.is_empty() || filter.from.is_some()) {
            return Err(invalid("syntax", "AND needs a filter on each side."));
        }
        Ok(filter)
    }
    pub fn matches(&self, operation: &Operation) -> bool {
        use jj_lib::object_id::ObjectId as _;
        let metadata = &operation.store_operation().metadata;
        self.terms.iter().all(|term| match term {
            Term::Text(field, value) => match field.as_str() {
                "description" => metadata.description.to_lowercase().contains(value),
                "user" => metadata.username.to_lowercase().contains(value),
                "host" => metadata.hostname.to_lowercase().contains(value),
                "workspace" => metadata
                    .workspace_name
                    .as_ref()
                    .is_some_and(|w| w.as_str().to_lowercase().contains(value)),
                _ => false,
            },
            Term::Id(value) => operation.id().hex().starts_with(value),
            Term::Snapshot(value) => metadata.is_snapshot == *value,
            Term::After(time) => metadata.time.end.timestamp.0 >= *time,
            Term::Before(time) => metadata.time.end.timestamp.0 < *time,
        })
    }
    pub async fn start(
        &self,
        repo: &ReadonlyRepo,
        work: impl FnMut() -> Result<()>,
    ) -> Result<Operation> {
        self.start_at(repo.operation(), work).await
    }
    pub fn needs_prefix_lookup(&self) -> bool {
        self.from
            .as_ref()
            .is_some_and(|value| !value.starts_with('@'))
    }
    pub async fn start_at(
        &self,
        head: &Operation,
        mut work: impl FnMut() -> Result<()>,
    ) -> Result<Operation> {
        let Some(selector) = &self.from else {
            return Ok(head.clone());
        };
        let selected =
            jj_lib::op_walk::resolve_op_at(&head.op_store(), std::slice::from_ref(head), selector)
                .await
                .map_err(|error| match error {
                    jj_lib::op_walk::OpsetEvaluationError::OpStore(error) => store_error(error),
                    _ => invalid(
                        "unresolved_symbol",
                        "Operation selector is missing or ambiguous in this history.",
                    ),
                })?;
        let heads = [head.clone()];
        let stream = jj_lib::op_walk::walk_ancestors(&heads);
        let mut stream = std::pin::pin!(stream);
        while let Some(operation) = stream.try_next().await.map_err(store_error)? {
            work()?;
            if operation.id() == selected.id() {
                return Ok(selected);
            }
        }
        Err(invalid(
            "unresolved_symbol",
            "from: must select an ancestor of the displayed snapshot.",
        ))
    }
}
pub(crate) fn store_error(error: jj_lib::op_store::OpStoreError) -> Error {
    match error {
        jj_lib::op_store::OpStoreError::Other(source) => source
            .downcast::<Error>()
            .map(|error| *error)
            .unwrap_or(Error::Unavailable),
        _ => Error::Unavailable,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_fields_quotes_dates_and_selectors() {
        let filter = Filter::parse("from:@-- user:Steve AND description:\"rebase branch\" after:2026-10-01 before:2026-11-01 snapshot:false").unwrap();
        assert_eq!(filter.from.as_deref(), Some("@--"));
        assert_eq!(filter.terms.len(), 5);
        assert!(matches!(&filter.terms[1], Term::Text(_, value) if value == "rebase branch"));
        for query in [
            "\"user:steve\"",
            "\"OR\"",
            "description:\"quoted \\\"word\\\"\"",
            "bare text",
        ] {
            assert!(Filter::parse(query).is_ok(), "{query}");
        }
    }
    #[test]
    fn rejects_invalid_queries_and_limits() {
        for query in [
            "unknown:x",
            "description:\"unfinished",
            "snapshot:yes",
            "after:2026-02-30",
            "from:@ from:@-",
            "id:xyz",
            "from:mutable()",
            "OR",
            "x OR y",
            "AND",
            "x AND",
            "AND x",
            "x AND AND y",
            "\"\"",
            "none()",
        ] {
            assert!(Filter::parse(query).is_err(), "{query}");
        }
        assert!(matches!(
            Filter::parse(&"x".repeat(4097)),
            Err(Error::Limit)
        ));
        assert!(matches!(Filter::parse(&"x ".repeat(65)), Err(Error::Limit)));
    }
}
