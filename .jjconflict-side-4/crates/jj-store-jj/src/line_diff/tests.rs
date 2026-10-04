use super::*;

fn diff(before: &str, after: &str, max: usize) -> Option<Vec<Span>> {
    bounded(before, after, max, |_| Ok::<_, ()>(())).unwrap()
}

fn reconstruct(before: &str, after: &str, spans: &[Span]) {
    assert_eq!(
        spans
            .iter()
            .map(|v| &before[v.before.clone()])
            .collect::<String>(),
        before
    );
    assert_eq!(
        spans
            .iter()
            .map(|v| &after[v.after.clone()])
            .collect::<String>(),
        after
    );
    for span in spans {
        if span.kind == DiffHunkKind::Matching {
            assert_eq!(before[span.before.clone()], after[span.after.clone()]);
        }
    }
}

#[test]
fn small_diffs_retain_jj_default_hunks_and_exact_bytes() {
    for (before, after) in [
        ("a\r\nb\nlast", "a\r\nc\nlast"),
        ("", "added"),
        ("last\n", "last"),
    ] {
        let spans = diff(before, after, 250_000).unwrap();
        reconstruct(before, after, &spans);
        let native = ContentDiff::by_line([before.as_bytes(), after.as_bytes()]);
        let native = native
            .hunks()
            .filter(|v| v.contents.iter().any(|v| !v.is_empty()))
            .collect::<Vec<_>>();
        assert_eq!(spans.len(), native.len());
        for (span, hunk) in spans.iter().zip(native) {
            assert_eq!(span.kind, hunk.kind);
            assert_eq!(before[span.before.clone()].as_bytes(), hunk.contents[0]);
            assert_eq!(after[span.after.clone()].as_bytes(), hunk.contents[1]);
        }
    }
}

#[test]
fn sparse_long_file_edits_with_insertions_and_final_newline_change() {
    let before = (0..2300)
        .map(|n| format!("line {n}\r\n"))
        .collect::<String>();
    let after = before
        .replace("line 20\r\n", "edited\r\ninserted\r\n")
        .replace("line 1100\r\n", "edited middle\r\n")
        .replace("line 2200\r\n", "edited end\r\n");
    let after = after.trim_end_matches("\r\n");
    let spans = diff(&before, after, 250_000).unwrap();
    reconstruct(&before, after, &spans);
    assert!(
        spans
            .iter()
            .filter(|v| v.kind == DiffHunkKind::Different)
            .count()
            >= 4
    );
}

#[test]
fn repeated_common_lines_are_trimmed_without_unique_anchors() {
    let before = format!(
        "{}before\r\n{}",
        "same\r\n".repeat(1100),
        "same\r\n".repeat(1100)
    );
    let after = before.replace("before", "after");
    let spans = diff(&before, &after, 250_000).unwrap();
    reconstruct(&before, &after, &spans);
}

#[test]
fn expensive_repeated_comparisons_refuse_complete_content() {
    assert!(diff(&"a\nb\n".repeat(600), &"b\na\n".repeat(600), 250_000).is_none());
    let before = format!(
        "{}anchor one\n{}anchor two\n{}",
        "a\n".repeat(100),
        "a\n".repeat(100),
        "a\n".repeat(100)
    );
    let after = before.replace("a\n", "b\n");
    assert!(
        diff(&before, &after, 15_000).is_none(),
        "work is cumulative across gaps"
    );
}

#[test]
fn request_budget_cancellation_propagates_without_a_partial_diff() {
    let before = (0..2300).map(|n| format!("line {n}\n")).collect::<String>();
    let after = before.replace("line 20", "edited");
    let mut calls = 0;
    let result = bounded(&before, &after, 250_000, |_| {
        calls += 1;
        if calls == 3 { Err("canceled") } else { Ok(()) }
    });
    assert!(matches!(result, Err("canceled")));
}
