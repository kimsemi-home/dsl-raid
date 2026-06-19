use super::super::fixture::update;
use serde_json::json;

#[test]
fn feedback_update_requires_evidence() {
    let mut value = super::manifest("applied");
    value["debts"][0]["updates"] = json!([update("applied", json!([]))]);

    assert_eq!(
        super::issues(&value),
        vec!["feedback update update:review-policy requires evidence"]
    );
}

#[test]
fn feedback_update_rejects_unknown_evidence() {
    let mut value = super::manifest("applied");
    value["debts"][0]["updates"] = json!([update("applied", json!(["evidence:missing"]))]);

    assert_eq!(
        super::issues(&value),
        vec!["feedback update update:review-policy references unknown evidence evidence:missing"]
    );
}
