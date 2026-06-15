pub(super) fn split_word_operator<'a>(
    clause: &'a str,
    operator: &str,
) -> Option<(&'a str, &'a str)> {
    let needle = format!(" {operator} ");
    clause
        .to_ascii_lowercase()
        .find(&needle)
        .map(|index| (&clause[..index], &clause[index + needle.len()..]))
}

pub(super) fn split_logical(input: &str, operator: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut quote: Option<char> = None;
    let mut bracket_depth = 0usize;
    let bytes = input.as_bytes();
    let needle = format!(" {operator} ");
    let lower = input.to_ascii_lowercase();
    let lower_bytes = lower.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        let character = input[index..].chars().next().unwrap_or_default();
        match character {
            '\'' | '"' if quote == Some(character) => quote = None,
            '\'' | '"' if quote.is_none() => quote = Some(character),
            '[' if quote.is_none() => bracket_depth += 1,
            ']' if quote.is_none() && bracket_depth > 0 => bracket_depth -= 1,
            _ => {}
        }
        if quote.is_none()
            && bracket_depth == 0
            && lower_bytes[index..].starts_with(needle.as_bytes())
        {
            parts.push(input[start..index].trim().to_string());
            index += needle.len();
            start = index;
            continue;
        }
        index += character.len_utf8();
    }
    parts.push(input[start..].trim().to_string());
    parts
}

pub(super) fn normalize_query_value(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

pub(super) fn parse_list_value(value: &str) -> Vec<String> {
    let value = value.trim().trim_start_matches('[').trim_end_matches(']');
    value
        .split(',')
        .map(normalize_query_value)
        .filter(|item| !item.is_empty())
        .collect()
}
