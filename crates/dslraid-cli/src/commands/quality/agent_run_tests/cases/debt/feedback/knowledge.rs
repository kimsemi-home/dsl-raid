use serde_json::json;

#[test]
fn review_debt_requires_policy_ontology_or_spec_update() {
    let mut value = super::manifest("applied");
    value["debts"][0]["updates"][0]["kind"] = json!("revalidation");

    assert_eq!(
        super::issues(&value),
        vec!["debt debt:review requires policy, ontology, or spec knowledge update"]
    );
}
