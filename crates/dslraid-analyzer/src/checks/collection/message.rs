use super::CollectionCheck;

pub(super) fn from_check(check: &CollectionCheck<'_>) -> &'static str {
    if check.failures.is_empty() {
        check.pass_message
    } else {
        check.fail_message
    }
}
