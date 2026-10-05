pub(crate) fn common_prefix(left: &[u8], right: &[u8]) -> usize {
    left.iter()
        .zip(right)
        .take_while(|(left, right)| left == right)
        .count()
}

pub(crate) fn common_suffix(left: &[u8], right: &[u8], prefix: usize) -> usize {
    left[prefix..]
        .iter()
        .rev()
        .zip(right[prefix.min(right.len())..].iter().rev())
        .take_while(|(left, right)| left == right)
        .count()
}

pub(crate) fn occurrences(haystack: &str, needle: &[u8], limit: usize) -> Vec<usize> {
    let bytes = haystack.as_bytes();
    if needle.is_empty() || needle.len() > bytes.len() {
        return Vec::new();
    }
    (0..=bytes.len() - needle.len())
        .filter(|offset| {
            haystack.is_char_boundary(*offset)
                && haystack.is_char_boundary(*offset + needle.len())
                && bytes[*offset..].starts_with(needle)
        })
        .take(limit.saturating_add(1))
        .collect()
}

pub(crate) fn floor_boundary(text: &str, mut offset: usize) -> usize {
    while offset > 0 && !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

pub(crate) fn ceil_boundary(text: &str, mut offset: usize) -> usize {
    while offset < text.len() && !text.is_char_boundary(offset) {
        offset += 1;
    }
    offset
}
