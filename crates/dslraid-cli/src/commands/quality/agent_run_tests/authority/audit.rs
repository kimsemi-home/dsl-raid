use super::super::fixtures::adversarial;
use super::super::fixtures::attach_producer_reliability;
use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn audit_scope_requires_human_review_and_capacity() {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["producer"]["trust_tier"] = json!("T3");
    value["authority_gate"]["scope"] = json!("audit");
    attach_producer_reliability(&mut value);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec![
            "authority scope audit requires human review",
            "high-risk authority requires review capacity receipt"
        ]
    );
}
