mod producer;
mod reliability;
mod review_capacity;
mod reviewer;

use serde_json::Value;

pub(super) fn push_producer_issues(value: &Value, issues: &mut Vec<String>) {
    producer::push_issues(value, issues);
}

pub(super) fn push_review_issues(value: &Value, issues: &mut Vec<String>) {
    reviewer::push_issues(value, issues);
    review_capacity::push_issues(value, issues);
}
