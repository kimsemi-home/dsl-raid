mod closure;

use super::fields::{field_is, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    if has_open_debt(value) {
        issues.push("approved run cannot carry open debt".to_string());
    }
    for debt in items(value, "debts") {
        closure::push_issues(value, debt, issues);
    }
}

fn has_open_debt(value: &Value) -> bool {
    items(value, "debts").any(|item| field_is(item, "status", "open"))
}
