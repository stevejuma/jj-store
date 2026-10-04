use super::*;
fn hit(n: usize) -> ContentHit {
    ContentHit {
        kind: ContentKind::Task,
        id: format!("{n:04}"),
        title: "Needle title".into(),
        excerpt: String::new(),
        actor: jj_store_model::PrincipalId::new([1; 16]),
        at_millis: n as i64,
        revision: "1".into(),
        thread: None,
        message: None,
        project: None,
        task: None,
        comment: None,
    }
}
#[test]
fn search_merges_root_orders_and_hydrates_only_the_selected_deep_window() {
    let mut sources = Vec::new();
    for domain in 0..2 {
        let mut index = Index::new(100);
        for n in 0..100 {
            let mut item = hit(n);
            item.revision = domain.to_string();
            index
                .add(
                    item,
                    &format!("{}needle", "İ".repeat(400)),
                    Field::Both,
                    n % 2 == 0,
                )
                .unwrap();
        }
        sources.push(Arc::new(index.finish()));
    }
    let query = ContentQuery {
        q: "needle".into(),
        ..Default::default()
    };
    let selection = Selection::new(sources.clone(), &query, "needle").unwrap();
    assert_eq!(selection.keys.len(), 200);
    let page = selection.page("needle", 49, 2);
    assert_eq!(
        (page[0].id.as_str(), page[0].revision.as_str()),
        ("0075", "1")
    );
    assert_eq!(
        (page[1].id.as_str(), page[1].revision.as_str()),
        ("0074", "0")
    );
    assert!(
        page.iter()
            .all(|hit| hit.excerpt.contains("needle") && hit.excerpt.starts_with('…'))
    );
    assert_eq!(
        sources
            .iter()
            .map(|s| s.hydrated.load(std::sync::atomic::Ordering::Relaxed))
            .sum::<usize>(),
        2
    );
    let recent = Selection::new(sources, &ContentQuery::default(), "").unwrap();
    assert_eq!(recent.keys.len(), 100);
    assert_eq!(recent.page("", 0, 1)[0].id, "0098");
}
#[test]
fn search_projection_refuses_body_growth_before_retaining_fields() {
    let mut index = Index::new(1);
    index.bytes = MAX_BYTES - 1;
    assert!(matches!(
        index.add(hit(1), "İ Needle", Field::Both, true),
        Err(Error::Limit)
    ));
    assert!(index.docs.is_empty());
}
