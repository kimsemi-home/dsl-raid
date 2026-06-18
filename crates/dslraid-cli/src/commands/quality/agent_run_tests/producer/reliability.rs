use super::super::fixtures::adversarial;
use super::super::fixtures::attach_producer_reliability;
use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn trusted_producer_requires_reliability_evidence() {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["producer"]["trust_tier"] = json!("T3");
    value["authority_gate"]["scope"] = json!("security");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("human:alice");

    assert!(super::super::super::agent_run::semantic_issues(&value)
        .contains(&"trusted producer agent:codex requires reliability evidence".to_string()));
}

#[test]
fn trusted_producer_accepts_reliability_evidence() {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["producer"]["trust_tier"] = json!("T3");
    attach_producer_reliability(&mut value);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}
