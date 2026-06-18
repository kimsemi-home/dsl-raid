pub(super) fn text(passed: bool) -> &'static str {
    if passed {
        "All semantic references resolve."
    } else {
        "Some semantic references do not resolve."
    }
}

pub(super) fn suggestion() -> &'static str {
    "Add the missing subject or update the reference to a stable existing ID."
}
