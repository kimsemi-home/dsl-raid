pub(super) fn from_passed(passed: bool) -> &'static str {
    if passed {
        "All states are reachable from the initial state."
    } else {
        "Some states are not reachable from the initial state."
    }
}
