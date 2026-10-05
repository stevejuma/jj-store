//! Service admission around JJ's default line diff, without a whole-file matrix.
use jj_lib::diff::{ContentDiff, DiffHunkKind};
use std::{collections::HashMap, ops::Range};

pub(crate) struct Span {
    pub kind: DiffHunkKind,
    pub before: Range<usize>,
    pub after: Range<usize>,
}

fn lines(text: &str) -> usize {
    text.bytes().filter(|&v| v == b'\n').count() + 1
}

fn unique<'a>(lines: &[&'a str]) -> HashMap<&'a str, Option<usize>> {
    let mut found = HashMap::new();
    for (index, &line) in lines.iter().enumerate() {
        found
            .entry(line)
            .and_modify(|v| *v = None)
            .or_insert(Some(index));
    }
    found
}

fn offsets(lines: &[&str]) -> Vec<usize> {
    std::iter::once(0)
        .chain(lines.iter().scan(0, |end, line| {
            *end += line.len();
            Some(*end)
        }))
        .collect()
}

fn append(spans: &mut Vec<Span>, span: Span) {
    if span.before.is_empty() && span.after.is_empty() {
        return;
    }
    if let Some(last) = spans.last_mut()
        && last.kind == span.kind
    {
        last.before.end = span.before.end;
        last.after.end = span.after.end;
    } else {
        spans.push(span);
    }
}

/// A complete result or a complexity refusal. No partial patch escapes.
pub(crate) fn bounded<E>(
    before: &str,
    after: &str,
    max_cells: usize,
    mut charge: impl FnMut(usize) -> Result<(), E>,
) -> Result<Option<Vec<Span>>, E> {
    charge(before.len() + after.len())?;
    let mut windows = vec![(false, 0..before.len(), 0..after.len())];
    if lines(before).saturating_mul(lines(after)) > max_cells {
        let left = before.split_inclusive('\n').collect::<Vec<_>>();
        let right = after.split_inclusive('\n').collect::<Vec<_>>();
        charge(left.len() + right.len())?;
        let leading = left.iter().zip(&right).take_while(|(a, b)| a == b).count();
        let trailing = left[leading..]
            .iter()
            .rev()
            .zip(right[leading..].iter().rev())
            .take_while(|(a, b)| a == b)
            .count();
        let left_end = left.len() - trailing;
        let right_end = right.len() - trailing;
        let left_unique = unique(&left[leading..left_end]);
        let right_unique = unique(&right[leading..right_end]);
        let left_offsets = offsets(&left);
        let right_offsets = offsets(&right);
        windows.clear();
        windows.push((true, 0..left_offsets[leading], 0..right_offsets[leading]));
        let (mut l, mut r) = (leading, leading);
        for (index, &line) in left[leading..left_end].iter().enumerate() {
            charge(1)?;
            if left_unique.get(line) != Some(&Some(index)) {
                continue;
            }
            let Some(Some(position)) = right_unique.get(line) else {
                continue;
            };
            let (a, b) = (leading + index, leading + position);
            if b < r {
                continue;
            }
            windows.push((
                false,
                left_offsets[l]..left_offsets[a],
                right_offsets[r]..right_offsets[b],
            ));
            windows.push((
                true,
                left_offsets[a]..left_offsets[a + 1],
                right_offsets[b]..right_offsets[b + 1],
            ));
            (l, r) = (a + 1, b + 1);
        }
        windows.push((
            false,
            left_offsets[l]..left_offsets[left_end],
            right_offsets[r]..right_offsets[right_end],
        ));
        windows.push((
            true,
            left_offsets[left_end]..before.len(),
            right_offsets[right_end]..after.len(),
        ));
    }
    let mut cells = 0usize;
    for (matching, a, b) in &windows {
        charge(1)?;
        if *matching || (a.is_empty() && b.is_empty()) {
            continue;
        }
        cells = cells
            .saturating_add(lines(&before[a.clone()]).saturating_mul(lines(&after[b.clone()])));
        if cells > max_cells {
            return Ok(None);
        }
    }
    charge(cells)?;
    let mut spans = vec![];
    for (matching, a, b) in windows {
        charge(1)?;
        if matching {
            append(
                &mut spans,
                Span {
                    kind: DiffHunkKind::Matching,
                    before: a,
                    after: b,
                },
            );
            continue;
        }
        let diff =
            ContentDiff::by_line([before[a.clone()].as_bytes(), after[b.clone()].as_bytes()]);
        let (mut l, mut r) = (a.start, b.start);
        for hunk in diff.hunks() {
            charge(1)?;
            let (le, re) = (l + hunk.contents[0].len(), r + hunk.contents[1].len());
            append(
                &mut spans,
                Span {
                    kind: hunk.kind,
                    before: l..le,
                    after: r..re,
                },
            );
            (l, r) = (le, re);
        }
    }
    Ok(Some(spans))
}

#[cfg(test)]
mod tests;
