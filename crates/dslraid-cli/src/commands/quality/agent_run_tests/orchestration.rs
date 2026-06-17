use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_requires_orchestration_receipt() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value.as_object_mut().unwrap().remove("orchestration");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires orchestration receipt"]
    );
}

#[test]
fn orchestration_receipt_must_match_manifest_routing() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["orchestration"]["selected_producer"] = json!("agent:other");
    value["orchestration"]["authority_profile"] = json!("automatic");
    value["orchestration"]["policy_hash"] = json!("sha256:other");
    value["orchestration"]["lease"] = json!("lease:other");
    value["orchestration"]["selected_reviewers"] = json!(["reviewer:missing"]);
    value["orchestration"]["input_evidence"] = json!(["evidence:missing"]);
    value["orchestration"]["output_artifacts"] = json!(["artifact:missing"]);
    value["orchestration"]["evidence"] = json!(["evidence:missing"]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "orchestration producer agent:other differs from manifest agent:codex",
            "orchestration authority profile automatic differs from manifest sidecar",
            "orchestration policy hash sha256:other differs from manifest sha256:policy",
            "orchestration lease lease:other differs from manifest lease:runscope-quality-001",
            "orchestration references unknown evidence evidence:missing",
            "orchestration references unknown reviewer reviewer:missing",
            "orchestration references unknown input evidence evidence:missing",
            "orchestration references unknown output artifact artifact:missing"
        ]
    );
}
