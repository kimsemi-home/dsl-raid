use serde_json::json;

#[test]
fn closed_debt_requires_feedback_update() {
    let mut value = super::manifest("applied");
    value["debts"][0]["updates"] = json!([]);

    assert_eq!(
        super::issues(&value),
        vec!["debt debt:review requires feedback closure update"]
    );
}
