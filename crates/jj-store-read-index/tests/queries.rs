use jj_store_read_index::{Index, Row};
use std::sync::Arc;
fn row(id: usize, group: usize, text: &str) -> Row {
    Row::new(
        id.to_string(),
        format!("source-{id}-{group}-{text}"),
        (id as u64).to_be_bytes().to_vec(),
    )
    .facet("group", group)
    .text(text)
}
#[test]
fn scoped_selection_inspects_only_posting_intersection() {
    let index = Index::build(
        (0..4_000)
            .map(|id| row(id, id % 100, "Alpha İstanbul Straße"))
            .collect(),
    )
    .unwrap();
    let selection = index.select(&[("group", "7".into())], "STAN");
    assert_eq!(selection.ids.len(), 40);
    assert_eq!(selection.inspected, 40);
    assert_eq!(selection.ids[0], "7");
    assert!(
        index.select(&[], "istanbul").ids.is_empty(),
        "Unicode lowercase expansion must preserve substring semantics"
    );
    assert_eq!(index.select(&[], "straße").ids.len(), 4_000);
    assert_eq!(index.select(&[("group", "absent".into())], "").inspected, 0);
}
#[test]
fn trigrams_verify_order_and_field_boundaries() {
    let index = Index::build(vec![
        row(1, 1, "abc xx bcd"),
        row(2, 1, "abcd"),
        row(3, 1, "abcdef"),
    ])
    .unwrap();
    assert_eq!(index.select(&[], "abcd").ids, ["2", "3"]);
    assert_eq!(index.select(&[], "bc").ids, ["1", "2", "3"]);
    assert_eq!(
        index
            .after(&index.select(&[], "abcd").ids, Some("2"))
            .unwrap()
            .collect::<Vec<_>>(),
        ["3"]
    );
}
#[test]
fn incremental_replacements_removals_and_compaction_match_full_build() {
    let mut rows: Vec<_> = (0..1_000).map(|id| row(id, id % 4, "original")).collect();
    let mut index = Arc::new(Index::build(rows.clone()).unwrap());
    let historical = index.clone();
    for iteration in 0..12 {
        rows.remove(0);
        rows[iteration] = row(iteration + 1000, 9, "replacement");
        let next = Index::advance(index, rows.clone()).unwrap();
        let full = Index::build(rows.clone()).unwrap();
        for text in ["", "original", "replacement", "a", "missing"] {
            for filters in [
                vec![],
                vec![("group", "9".into())],
                vec![("group", "2".into())],
            ] {
                assert_eq!(
                    next.select(&filters, text).ids,
                    full.select(&filters, text).ids
                );
            }
        }
        if iteration == 0 {
            assert_eq!(
                next.rows().len(),
                1,
                "only the replacement needs new normalization/postings"
            );
        }
        index = Arc::new(next);
    }
    assert_eq!(historical.select(&[], "original").ids.len(), 1_000);
    assert_eq!(historical.select(&[], "replacement").ids.len(), 0);
}
#[test]
fn duplicate_ids_and_oversized_input_are_rejected() {
    assert!(Index::build(vec![row(1, 1, "a"), row(1, 2, "b")]).is_err());
    assert!(
        Index::build(
            (0..=jj_store_read_index::MAX_ROWS)
                .map(|id| row(id, 1, ""))
                .collect()
        )
        .is_err()
    );
}
