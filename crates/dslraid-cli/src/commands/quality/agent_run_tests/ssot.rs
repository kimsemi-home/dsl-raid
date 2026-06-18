use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_rejects_missing_ssot_revalidation() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["ssot"]
        .as_object_mut()
        .unwrap()
        .remove("revalidation");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires ssot revalidation status"]
    );
}

#[test]
fn approved_manifest_rejects_expired_ssot_revalidation() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["ssot"]["revalidation"]["status"] = json!("expired");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run cannot use ssot revalidation status expired"]
    );
}

#[test]
fn approved_manifest_rejects_incomplete_ssot_revalidation() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["ssot"]["revalidation"] = json!({"status": "valid"});

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "ssot revalidation requires assessed_at",
            "ssot revalidation requires assessor",
            "ssot revalidation requires revalidate_at"
        ]
    );
}

#[test]
fn approved_manifest_rejects_unknown_ssot_revalidation_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["ssot"]["revalidation"]["evidence"] = json!("evidence:missing");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["ssot revalidation references unknown evidence evidence:missing"]
    );
}

#[test]
fn ssot_defect_claim_requires_governance_plan_and_freeze() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([{
        "id": "claim:ssot-defect",
        "subject": "agent-run:runscope-quality-001",
        "statement": "SSOT Defect is confirmed in the canonical IR.",
        "confidence": "medium",
        "assessor": "sidecar:dslraid-quality",
        "interpreted_under": "0.1.0",
        "status": "supported",
        "evidence": ["evidence:quality"]
    }]);
    let issues = super::super::agent_run::semantic_issues(&value);
    let expected = [
        "ssot defect claim claim:ssot-defect requires authority governance scope",
        "ssot defect claim claim:ssot-defect requires verification plan",
        "ssot defect claim claim:ssot-defect requires open quarantine containment",
    ];
    assert_eq!(issues, expected);
}
