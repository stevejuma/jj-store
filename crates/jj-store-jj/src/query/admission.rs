//! Shape admission before native metadata allocation; never a persistence codec.
use super::{Error, Result, access::Budget};
pub use crate::read_support::MetadataLimits as Limits;
use jj_store_model::{ObjectKey, ObjectKind, envelope};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use std::fmt;

#[derive(Clone, Copy)]
enum Shape {
    Value,
    Commit,
    Operation,
    Tree,
    Copy,
    View,
    References,
    Remotes,
    RemoteView,
    RemoteReferences,
    RemoteReference,
    Predecessors,
    Merge,
    Edges,
    Entries,
}
impl Shape {
    fn field(self, name: &str) -> Self {
        match (self, name) {
            (Self::Commit, "root_tree" | "conflict_labels") => Self::Merge,
            (Self::Commit | Self::Copy | Self::Operation, "parents")
            | (Self::Commit, "predecessors") => Self::Edges,
            (Self::Operation, "commit_predecessors") => Self::Predecessors,
            (Self::Predecessors, _) => Self::Edges,
            (Self::Tree, "entries") => Self::Entries,
            (Self::View, "local_bookmarks" | "local_tags" | "git_refs" | "git_heads") => {
                Self::References
            }
            (Self::References, _) | (Self::RemoteReference, "target") => Self::Merge,
            (Self::View, "remote_views") => Self::Remotes,
            (Self::Remotes, _) => Self::RemoteView,
            (Self::RemoteView, "bookmarks" | "tags") => Self::RemoteReferences,
            (Self::RemoteReferences, _) => Self::RemoteReference,
            _ => Self::Value,
        }
    }
    fn count_limit(self, limits: Limits) -> usize {
        match self {
            Self::Merge => limits.max_merge_terms,
            Self::Edges => limits.max_parent_edges,
            Self::Entries => limits.max_tree_entries,
            _ => usize::MAX, // Every element still consumes the cumulative node limit.
        }
    }
}
pub(super) fn check(key: ObjectKey, bytes: &[u8], budget: &Budget) -> Result<()> {
    let payload = envelope::decode(bytes, key, crate::codec::MAX_METADATA_BYTES)
        .map_err(|_| budget.fail(Error::Corrupt))?;
    let shape = match key.kind() {
        ObjectKind::Commit => Shape::Commit,
        ObjectKind::Operation => Shape::Operation,
        ObjectKind::Tree => Shape::Tree,
        ObjectKind::Copy => Shape::Copy,
        ObjectKind::View => Shape::View,
        ObjectKind::Symlink => return budget.metadata_node(), // Raw UTF-8, not JSON.
        ObjectKind::File => return Err(budget.fail(Error::Invalid)),
    };
    let mut decoder = serde_json::Deserializer::from_slice(payload);
    let result = Scan {
        budget,
        shape,
        depth: 0,
    }
    .deserialize(&mut decoder)
    .and_then(|()| decoder.end());
    result.map_err(|_| {
        budget
            .failure()
            .unwrap_or_else(|| budget.fail(Error::Corrupt))
    })
}

// Scan string/number scalars without retaining them. serde_json may use scratch
// for a single escaped string; the already enforced metadata byte cap bounds it.
struct Scan<'a> {
    budget: &'a Budget,
    shape: Shape,
    depth: usize,
}
impl<'de> DeserializeSeed<'de> for Scan<'_> {
    type Value = ();
    fn deserialize<D: de::Deserializer<'de>>(
        self,
        decoder: D,
    ) -> std::result::Result<(), D::Error> {
        self.budget.metadata_node().map_err(de::Error::custom)?;
        if self.depth > 32 {
            return Err(de::Error::custom(self.budget.fail(Error::Limit)));
        }
        decoder.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Scan<'_> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("bounded native metadata")
    }
    fn visit_bool<E: de::Error>(self, _: bool) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_u64<E: de::Error>(self, _: u64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_str<E: de::Error>(self, _: &str) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_unit<E: de::Error>(self) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> std::result::Result<(), A::Error> {
        let limit = self.shape.count_limit(self.budget.limits.metadata);
        let mut count = 0;
        while seq
            .next_element_seed(Scan {
                budget: self.budget,
                shape: Shape::Value,
                depth: self.depth + 1,
            })?
            .is_some()
        {
            count += 1;
            if count > limit {
                return Err(de::Error::custom(self.budget.fail(Error::Limit)));
            }
        }
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> std::result::Result<(), A::Error> {
        while let Some(shape) = map.next_key_seed(Field {
            budget: self.budget,
            shape: self.shape,
        })? {
            map.next_value_seed(Scan {
                budget: self.budget,
                shape,
                depth: self.depth + 1,
            })?;
        }
        Ok(())
    }
}
struct Field<'a> {
    budget: &'a Budget,
    shape: Shape,
}
impl<'de> DeserializeSeed<'de> for Field<'_> {
    type Value = Shape;
    fn deserialize<D: de::Deserializer<'de>>(
        self,
        decoder: D,
    ) -> std::result::Result<Shape, D::Error> {
        // Count in visit_str, not here: MapAccess can attempt an absent final key.
        decoder.deserialize_str(self)
    }
}
impl<'de> Visitor<'de> for Field<'_> {
    type Value = Shape;
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("metadata field")
    }
    fn visit_str<E: de::Error>(self, name: &str) -> std::result::Result<Shape, E> {
        self.budget.metadata_node().map_err(de::Error::custom)?;
        Ok(self.shape.field(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jj_store_model as m;
    use serde_json::json;

    fn budget(limits: Limits) -> Budget {
        Budget::new(super::super::Limits {
            metadata: limits,
            ..Default::default()
        })
        .unwrap()
    }
    fn scan(kind: ObjectKind, payload: &serde_json::Value, budget: &Budget) -> Result<()> {
        let key = ObjectKey::from_parts(kind as u8, [7; 64]).unwrap();
        let bytes = envelope::encode(
            key,
            &serde_json::to_vec(payload).unwrap(),
            crate::codec::MAX_METADATA_BYTES,
        )
        .unwrap();
        check(key, &bytes, budget)
    }
    #[test]
    fn query_shape_bounds_every_persisted_reference_and_edge_collection() {
        let terms = vec![json!(null); 5];
        let cases = [
            (ObjectKind::Commit, json!({"root_tree": terms})),
            (ObjectKind::Commit, json!({"conflict_labels": terms})),
            (ObjectKind::Commit, json!({"parents": terms})),
            (ObjectKind::Commit, json!({"predecessors": terms})),
            (ObjectKind::Copy, json!({"parents": terms})),
            (ObjectKind::Operation, json!({"parents": terms})),
            (
                ObjectKind::Operation,
                json!({"commit_predecessors": {"id": terms}}),
            ),
            (
                ObjectKind::View,
                json!({"local_bookmarks": {"name": terms}}),
            ),
            (ObjectKind::View, json!({"local_tags": {"name": terms}})),
            (ObjectKind::View, json!({"git_refs": {"name": terms}})),
            (ObjectKind::View, json!({"git_heads": {"workspace": terms}})),
            (
                ObjectKind::View,
                json!({"remote_views": {"remote": {"bookmarks": {"name": {"target": terms}}}}}),
            ),
            (
                ObjectKind::View,
                json!({"remote_views": {"remote": {"tags": {"name": {"target": terms}}}}}),
            ),
            (ObjectKind::Tree, json!({"entries": terms})),
        ];
        for (kind, value) in cases {
            let restricted = budget(Limits {
                max_merge_terms: 3,
                max_parent_edges: 3,
                max_tree_entries: 3,
                ..Default::default()
            });
            assert_eq!(
                scan(kind, &value, &restricted),
                Err(Error::Limit),
                "{kind:?}: {value}"
            );
            let allowed = budget(Limits {
                max_merge_terms: 5,
                max_parent_edges: 5,
                max_tree_entries: 5,
                ..Default::default()
            });
            scan(kind, &value, &allowed).unwrap();
        }
    }
    #[test]
    fn query_shape_node_limit_is_cumulative_and_counts_keys_nulls_and_empty_values() {
        let budget = budget(Limits {
            max_nodes: 14,
            ..Default::default()
        });
        let value = json!({"head_ids":[null,null],"local_tags":{}}); // 7 values/keys
        scan(ObjectKind::View, &value, &budget).unwrap();
        assert_eq!(budget.stats().metadata_nodes, 7);
        scan(ObjectKind::View, &value, &budget).unwrap();
        assert_eq!(budget.stats().metadata_nodes, 14);
        assert_eq!(scan(ObjectKind::View, &value, &budget), Err(Error::Limit));
        assert_eq!(budget.stats().metadata_nodes, 14);
    }
    #[test]
    fn query_shape_rejects_corruption_depth_and_cancellation_without_collections() {
        let key = m::ObjectKey::Commit(m::CommitId::new([7; 64]));
        let bad = envelope::encode(key, b"{\"parents\":[", 1024).unwrap();
        assert_eq!(
            check(key, &bad, &budget(Limits::default())),
            Err(Error::Corrupt)
        );
        let mut nested = json!(null);
        for _ in 0..34 {
            nested = json!([nested]);
        }
        assert_eq!(
            scan(ObjectKind::Commit, &nested, &budget(Limits::default())),
            Err(Error::Limit)
        );
        let canceled = budget(Limits::default());
        canceled.cancellation.cancel();
        assert_eq!(
            scan(ObjectKind::Commit, &json!({}), &canceled),
            Err(Error::Canceled)
        );
        let mut corrupt = envelope::encode(key, b"{}", 1024).unwrap();
        *corrupt.last_mut().unwrap() ^= 1;
        assert_eq!(
            check(key, &corrupt, &budget(Limits::default())),
            Err(Error::Corrupt)
        );
    }
    #[test]
    fn query_shape_recognizes_escaped_fields_and_rejects_trailing_input() {
        let key = m::ObjectKey::Commit(m::CommitId::new([7; 64]));
        let escaped = envelope::encode(
            key,
            br#"{"root\u005ftree":[null,null,null,null,null]}"#,
            1024,
        )
        .unwrap();
        assert_eq!(
            check(
                key,
                &escaped,
                &budget(Limits {
                    max_merge_terms: 3,
                    ..Default::default()
                })
            ),
            Err(Error::Limit)
        );
        let trailing = envelope::encode(key, b"{} {}", 1024).unwrap();
        assert_eq!(
            check(key, &trailing, &budget(Limits::default())),
            Err(Error::Corrupt)
        );
    }
}
