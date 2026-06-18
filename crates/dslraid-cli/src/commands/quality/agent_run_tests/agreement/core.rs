use super::super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn approved_manifest_requires_agreement() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["agreements"] = json!([]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires cross-agent agreement"]
    );
}

#[test]
fn agreement_requires_independent_reviewer_participant() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["agreements"] = json!([agreement(json!(["agent:codex", "agent:other"]))]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["agreement agreement:quality requires independent reviewer participant"]
    );
}

#[test]
fn agreement_rejects_unknown_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = agreement(json!(["agent:codex", "reviewer:quality"]));
    item["evidence"] = json!(["evidence:missing"]);
    value["agreements"] = json!([item]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["agreement agreement:quality references unknown evidence evidence:missing"]
    );
}

#[test]
fn agreement_requires_same_ontology() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = agreement(json!(["agent:codex", "reviewer:quality"]));
    item["interpreted_under"] = json!("9.9.9");
    value["agreements"] = json!([item]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["agreement agreement:quality interpreted_under differs from ssot ontology"]
    );
}

fn agreement(participants: Value) -> Value {
    json!({
        "id": "agreement:quality",
        "subject": "agent-run:runscope-quality-001",
        "participants": participants,
        "decision": "agree",
        "interpreted_under": "0.1.0",
        "evidence": ["evidence:quality"]
    })
}
