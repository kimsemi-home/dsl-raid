pub(super) fn find(lines: &[&str], start: usize, end: usize, pattern: &str) -> Option<usize> {
    if start == 0 || end < start {
        return None;
    }
    lines
        .iter()
        .enumerate()
        .skip(start - 1)
        .take(end - start + 1)
        .find(|(_, line)| line.contains(pattern))
        .map(|(index, _)| index + 1)
}

pub(super) fn span_end<T>(starts: &[(T, usize)], position: usize, total: usize) -> usize {
    starts
        .get(position + 1)
        .map(|next| next.1.saturating_sub(2))
        .unwrap_or(total)
}
