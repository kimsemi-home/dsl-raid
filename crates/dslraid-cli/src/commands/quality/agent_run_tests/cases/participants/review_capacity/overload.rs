use super::fixture::{capacity, high_risk_manifest};
use crate::commands::quality::agent_run;
use serde_json::json;

#[test]
fn queue_overflow_freezes_high_risk_sidecar_authority() {
    let mut value = high_risk_manifest();
    value["review_capacity"] = capacity("available", 6, 5, json!(["evidence:quality"]));

    assert_eq!(
        agent_run::semantic_issues(&value),
        vec![
            "review capacity queue depth exceeds max",
            "review capacity queue overflow freezes high-risk sidecar authority"
        ]
    );
}
