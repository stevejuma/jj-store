use jj_store_read_index::{Index, Row};
use std::time::Instant;
fn percentile(mut samples: Vec<u128>, percent: usize) -> u128 {
    samples.sort();
    samples[(samples.len() - 1) * percent / 100]
}
#[test]
fn measured_scoped_query_costs_at_small_medium_and_near_limit_sizes() {
    for size in [100, 1_000, 4_096, 9_500] {
        let rows: Vec<_> = (0..size)
            .map(|id| {
                Row::new(
                    id.to_string(),
                    format!("version-{id}"),
                    (id as u64).to_be_bytes().to_vec(),
                )
                .facet("project", id % 100)
                .text("This is a searchable recorded body")
            })
            .collect();
        let started = Instant::now();
        let index = Index::build(rows.clone()).unwrap();
        let cold_us = started.elapsed().as_micros();
        let mut scans = vec![];
        let mut indexed = vec![];
        for _ in 0..100 {
            let start = Instant::now();
            let baseline: Vec<_> = rows
                .iter()
                .filter(|r| {
                    r.facets["project"] == ["7"]
                        && r.text
                            .iter()
                            .any(|v| v.to_lowercase().contains("searchable"))
                })
                .map(|r| r.id.clone())
                .collect();
            scans.push(start.elapsed().as_nanos());
            let start = Instant::now();
            let selected = index.select(&[("project", "7".into())], "searchable");
            indexed.push(start.elapsed().as_nanos());
            assert_eq!(selected.ids, baseline);
            assert_eq!(selected.inspected, selected.ids.len());
        }
        println!(
            "rows={size} cold_build_us={cold_us} retained_bytes={} scan_p50_ns={} scan_p95_ns={} indexed_p50_ns={} indexed_p95_ns={} inspected={} baseline_inspected={size}",
            index.bytes(),
            percentile(scans.clone(), 50),
            percentile(scans, 95),
            percentile(indexed.clone(), 50),
            percentile(indexed, 95),
            index
                .select(&[("project", "7".into())], "searchable")
                .inspected
        );
        let mut changed = rows;
        changed[7].source = "edited-version".into();
        changed[7].text = vec!["A replacement body".into()];
        let start = Instant::now();
        let incremental = Index::advance(std::sync::Arc::new(index), changed.clone()).unwrap();
        let transition_us = start.elapsed().as_micros();
        let start = Instant::now();
        let rebuilt = Index::build(changed).unwrap();
        let rebuild_us = start.elapsed().as_micros();
        assert_eq!(
            incremental.rows().len(),
            1,
            "only the replacement is retokenized"
        );
        for query in ["searchable", "replacement"] {
            assert_eq!(
                incremental.select(&[("project", "7".into())], query).ids,
                rebuilt.select(&[("project", "7".into())], query).ids
            );
        }
        println!(
            "rows={size} transition_us={transition_us} rebuild_us={rebuild_us} retokenized_rows={} transition_retained_bytes={}",
            incremental.rows().len(),
            incremental.bytes()
        );
    }
}
