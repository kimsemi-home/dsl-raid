pub(crate) fn rust_type(input: &str) -> String {
    input
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect()
}

pub(crate) fn go_type(input: &str) -> String {
    rust_type(input)
}

pub(crate) fn snake(input: &str) -> String {
    input.replace(['-', '.'], "_")
}

pub(crate) fn camel(input: &str) -> String {
    let name = rust_type(input);
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_ascii_lowercase().to_string() + chars.as_str(),
        None => name,
    }
}

fn capitalize(part: &str) -> String {
    let mut chars = part.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_pascalized() {
        assert_eq!(rust_type("runtime-fsm"), "RuntimeFsm");
        assert_eq!(snake("runtime.fsm"), "runtime_fsm");
    }
}
