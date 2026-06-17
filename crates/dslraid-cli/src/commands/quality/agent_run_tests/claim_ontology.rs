use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn claim_interpreter_must_match_ssot_ontology() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([{
        "id": "claim:fresh-artifacts",
        "subject": "agent-run:runscope-quality-001",
        "statement": "Generated artifacts match the canonical IR.",
        "confidence": "medium",
        "assessor": "sidecar:dslraid-quality",
        "interpreted_under": "9.9.9",
        "status": "supported",
        "evidence": ["evidence:quality"]
    }]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["claim claim:fresh-artifacts interpreted_under 9.9.9 differs from ssot 0.1.0"]
    );
}
