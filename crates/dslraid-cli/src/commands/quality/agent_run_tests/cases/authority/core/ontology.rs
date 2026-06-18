use super::super::super::fixtures::adversarial;
use super::super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn ontology_scope_requires_human_review() {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["authority_gate"]["scope"] = json!("ontology");

    assert_eq!(
        super::super::super::super::agent_run::semantic_issues(&value),
        vec![
            "authority scope ontology requires human review",
            "high-risk authority requires review capacity receipt"
        ]
    );
}
