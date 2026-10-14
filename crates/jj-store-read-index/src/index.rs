use crate::{Error, MAX_BYTES, MAX_ROWS, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

/// Exact source identity plus viewer-independent ordering and lookup facts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Row {
    pub id: String,
    pub source: String,
    pub order: Vec<u8>,
    pub facets: BTreeMap<String, Vec<String>>,
    pub text: Vec<String>,
}
impl Row {
    pub fn new(id: String, source: String, order: Vec<u8>) -> Self {
        Self {
            id,
            source,
            order,
            facets: BTreeMap::new(),
            text: vec![],
        }
    }
    pub fn facet(mut self, name: &str, value: impl ToString) -> Self {
        self.facets
            .entry(name.into())
            .or_default()
            .push(value.to_string());
        self
    }
    pub fn text(mut self, value: &str) -> Self {
        self.text.push(value.to_owned());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Index {
    base: Option<Arc<Index>>,
    removed: BTreeSet<String>,
    depth: usize,
    rows: Vec<Row>,
    lookup: BTreeMap<String, usize>,
    facets: BTreeMap<String, BTreeMap<String, BTreeSet<usize>>>,
    trigrams: BTreeMap<String, BTreeSet<usize>>,
    folded: Vec<Vec<String>>,
    bytes: usize,
}
pub struct Selection {
    pub ids: Vec<String>,
    /// Rows whose exact predicates were inspected, excluding posting lookups.
    pub inspected: usize,
}
fn grams(text: &str) -> impl Iterator<Item = String> + '_ {
    let mut chars = text.chars();
    let mut first = chars.next();
    let mut second = chars.next();
    std::iter::from_fn(move || {
        let last = chars.next()?;
        let gram = [first?, second?, last].iter().collect();
        first = second;
        second = Some(last);
        Some(gram)
    })
}
impl Index {
    pub fn build(mut rows: Vec<Row>) -> Result<Self> {
        if rows.len() > MAX_ROWS {
            return Err(Error::Limit);
        }
        let source_bytes = crate::codec::encode(&rows)?.len();
        if source_bytes > MAX_BYTES {
            return Err(Error::Limit);
        }
        rows.sort_by(|a, b| a.order.cmp(&b.order).then(a.id.cmp(&b.id)));
        let mut index = Self {
            base: None,
            removed: BTreeSet::new(),
            depth: 0,
            rows,
            lookup: BTreeMap::new(),
            facets: BTreeMap::new(),
            trigrams: BTreeMap::new(),
            folded: vec![],
            bytes: source_bytes,
        };
        for (rank, row) in index.rows.iter().enumerate() {
            if row.id.is_empty()
                || row.source.is_empty()
                || index.lookup.insert(row.id.clone(), rank).is_some()
            {
                return Err(Error::Invalid);
            }
            index.bytes = index
                .bytes
                .checked_add(row.id.len() + 128)
                .ok_or(Error::Limit)?;
            for (name, values) in &row.facets {
                for value in values {
                    index.bytes = index
                        .bytes
                        .checked_add(name.len() + value.len() + 128)
                        .ok_or(Error::Limit)?;
                    if index.bytes > MAX_BYTES {
                        return Err(Error::Limit);
                    }
                    index
                        .facets
                        .entry(name.clone())
                        .or_default()
                        .entry(value.clone())
                        .or_default()
                        .insert(rank);
                }
            }
            for field in &row.text {
                let len = field
                    .chars()
                    .flat_map(char::to_lowercase)
                    .map(char::len_utf8)
                    .sum::<usize>();
                index.bytes = index.bytes.checked_add(len).ok_or(Error::Limit)?;
                if index.bytes > MAX_BYTES {
                    return Err(Error::Limit);
                }
            }
            let fields: Vec<_> = row.text.iter().map(|v| v.to_lowercase()).collect();
            let mut seen = BTreeSet::new();
            for field in &fields {
                for gram in grams(field) {
                    if !seen.contains(&gram) {
                        index.bytes = index
                            .bytes
                            .checked_add(gram.len() + 128)
                            .ok_or(Error::Limit)?;
                        if index.bytes > MAX_BYTES {
                            return Err(Error::Limit);
                        }
                        seen.insert(gram);
                    }
                }
            }
            for gram in seen {
                index.trigrams.entry(gram).or_default().insert(rank);
            }
            if index.bytes > MAX_BYTES {
                return Err(Error::Limit);
            }
            index.folded.push(fields);
        }
        Ok(index)
    }
    pub fn bytes(&self) -> usize {
        self.bytes
    }
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }
    pub fn all_rows(&self) -> Vec<Row> {
        let mut rows = self.base.as_ref().map_or(vec![], |base| base.all_rows());
        rows.retain(|r| !self.removed.contains(&r.id));
        rows.extend(self.rows.iter().cloned());
        rows
    }
    pub(crate) fn validate(&self) -> Result<()> {
        if self.depth > 4
            || self.rows.len() > MAX_ROWS
            || self.lookup.len() != self.rows.len()
            || self.folded.len() != self.rows.len()
            || self.bytes > MAX_BYTES
        {
            return Err(Error::Invalid);
        }
        if let Some(base) = &self.base {
            base.validate()?;
            if self.depth != base.depth + 1 || self.removed.iter().any(|id| base.row(id).is_none())
            {
                return Err(Error::Invalid);
            }
        } else if self.depth != 0 || !self.removed.is_empty() {
            return Err(Error::Invalid);
        }
        let mut expected_bytes = crate::codec::encode(&self.rows)?.len();
        if let Some(base) = &self.base {
            expected_bytes +=
                base.bytes() + self.removed.iter().map(|id| id.len() + 64).sum::<usize>();
        }
        let mut facet_entries = 0usize;
        let mut gram_entries = 0usize;
        for (rank, row) in self.rows.iter().enumerate() {
            expected_bytes += row.id.len() + 128;
            expected_bytes += row
                .facets
                .iter()
                .map(|(name, values)| {
                    values
                        .iter()
                        .map(|value| name.len() + value.len() + 128)
                        .sum::<usize>()
                })
                .sum::<usize>();
            if self.lookup.get(&row.id) != Some(&rank)
                || self.base.as_ref().is_some_and(|base| {
                    base.row(&row.id).is_some() && !self.removed.contains(&row.id)
                })
            {
                return Err(Error::Invalid);
            }
            if rank > 0
                && (
                    self.rows[rank - 1].order.as_slice(),
                    self.rows[rank - 1].id.as_str(),
                ) >= (row.order.as_slice(), row.id.as_str())
            {
                return Err(Error::Invalid);
            }
            let folded: Vec<_> = row.text.iter().map(|v| v.to_lowercase()).collect();
            if self.folded[rank] != folded {
                return Err(Error::Invalid);
            }
            expected_bytes += folded.iter().map(String::len).sum::<usize>();
            let unique: BTreeSet<_> = folded.iter().flat_map(|v| grams(v)).collect();
            expected_bytes += unique.iter().map(|gram| gram.len() + 128).sum::<usize>();
            for gram in &unique {
                if !self.trigrams.get(gram).is_some_and(|p| p.contains(&rank)) {
                    return Err(Error::Invalid);
                }
            }
            gram_entries += unique.len();
            for (name, values) in &row.facets {
                let unique: BTreeSet<_> = values.iter().collect();
                for value in &unique {
                    if !self
                        .facets
                        .get(name)
                        .and_then(|p| p.get(*value))
                        .is_some_and(|p| p.contains(&rank))
                    {
                        return Err(Error::Invalid);
                    }
                }
                facet_entries += unique.len();
            }
        }
        if expected_bytes != self.bytes {
            return Err(Error::Invalid);
        }
        if facet_entries
            != self
                .facets
                .values()
                .flat_map(BTreeMap::values)
                .map(BTreeSet::len)
                .sum::<usize>()
            || gram_entries != self.trigrams.values().map(BTreeSet::len).sum::<usize>()
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    pub fn row(&self, id: &str) -> Option<&Row> {
        if let Some(rank) = self.lookup.get(id) {
            return self.rows.get(*rank);
        }
        if self.removed.contains(id) {
            return None;
        }
        self.base.as_ref().and_then(|base| base.row(id))
    }
    /// Replacements/removals mask prior facts. Only changed rows are normalized
    /// and receive new postings; bounded depth periodically compacts a generation.
    pub fn advance(base: Arc<Self>, rows: Vec<Row>) -> Result<Self> {
        if rows.len() > MAX_ROWS {
            return Err(Error::Limit);
        }
        if base.depth >= 4 {
            return Self::build(rows);
        }
        let current: BTreeMap<_, _> = rows.iter().map(|r| (r.id.clone(), r)).collect();
        if current.len() != rows.len() {
            return Err(Error::Invalid);
        }
        let removed = base
            .all_rows()
            .into_iter()
            .filter(|r| current.get(&r.id).is_none_or(|new| *new != r))
            .map(|r| r.id)
            .collect::<BTreeSet<_>>();
        let changed = rows
            .into_iter()
            .filter(|r| base.row(&r.id) != Some(r))
            .collect();
        let mut index = Self::build(changed)?;
        index.bytes = index
            .bytes
            .checked_add(base.bytes())
            .and_then(|n| n.checked_add(removed.iter().map(|id| id.len() + 64).sum::<usize>()))
            .ok_or(Error::Limit)?;
        if index.bytes > MAX_BYTES {
            return Err(Error::Limit);
        }
        index.depth = base.depth + 1;
        index.removed = removed;
        index.base = Some(base);
        Ok(index)
    }
    pub fn select(&self, filters: &[(&str, String)], text: &str) -> Selection {
        let folded = text.trim().to_lowercase();
        let mut postings = Vec::new();
        for (field, value) in filters {
            let Some(posting) = self.facets.get(*field).and_then(|p| p.get(value)) else {
                return self.merge(
                    Selection {
                        ids: vec![],
                        inspected: 0,
                    },
                    filters,
                    text,
                );
            };
            postings.push(posting);
        }
        for gram in grams(&folded) {
            let Some(posting) = self.trigrams.get(&gram) else {
                return self.merge(
                    Selection {
                        ids: vec![],
                        inspected: 0,
                    },
                    filters,
                    text,
                );
            };
            postings.push(posting);
        }
        postings.sort_by_key(|p| p.len());
        let candidates: Box<dyn Iterator<Item = usize> + '_> = match postings.first() {
            Some(first) => Box::new(first.iter().copied()),
            None => Box::new(0..self.rows.len()),
        };
        let mut result = Selection {
            ids: vec![],
            inspected: 0,
        };
        for rank in candidates {
            if postings.iter().any(|p| !p.contains(&rank)) {
                continue;
            }
            result.inspected += 1;
            if folded.is_empty() || self.folded[rank].iter().any(|v| v.contains(&folded)) {
                result.ids.push(self.rows[rank].id.clone());
            }
        }
        self.merge(result, filters, text)
    }
    fn merge(&self, mut result: Selection, filters: &[(&str, String)], text: &str) -> Selection {
        if let Some(base) = &self.base {
            let previous = base.select(filters, text);
            result.inspected += previous.inspected;
            let mut old = previous
                .ids
                .into_iter()
                .filter(|id| !self.removed.contains(id))
                .peekable();
            let incoming = std::mem::take(&mut result.ids);
            for id in incoming {
                let row = self.row(&id).expect("current indexed row");
                while let Some(before) = old.peek() {
                    let prior = base.row(before).expect("prior indexed row");
                    if (row.order.as_slice(), row.id.as_str())
                        < (prior.order.as_slice(), prior.id.as_str())
                    {
                        break;
                    }
                    result.ids.push(old.next().expect("peeked row"));
                }
                result.ids.push(id);
            }
            result.ids.extend(old);
        }
        result
    }
    pub fn after<'a>(
        &'a self,
        ids: &'a [String],
        after: Option<&str>,
    ) -> Result<impl Iterator<Item = &'a str>> {
        let start = match after {
            None => 0,
            Some(id) => ids.iter().position(|v| v == id).ok_or(Error::Invalid)? + 1,
        };
        Ok(ids[start..].iter().map(String::as_str))
    }
    pub(crate) fn fingerprint(rows: &[Row]) -> Result<[u8; 32]> {
        use sha2::{Digest, Sha256};
        let mut ordered: Vec<_> = rows.iter().collect();
        ordered.sort_by(|a, b| a.id.cmp(&b.id));
        let bytes = crate::codec::encode(&ordered)?;
        if bytes.len() > MAX_BYTES {
            return Err(Error::Limit);
        }
        Ok(Sha256::digest(&bytes).into())
    }
}
