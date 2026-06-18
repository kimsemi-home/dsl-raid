use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn approved_manifest_rejects_lossy_translation_without_ledger() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["translations"] = json!([translation("lossy", "target", true, json!([]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["lossy translation translation:lisp-to-ir requires loss ledger"]
    );
}

#[test]
fn approved_manifest_rejects_forbidden_loss() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["translations"] = json!([translation("lossy", "target", false, json!([loss("L4")]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["translation translation:lisp-to-ir contains forbidden loss loss:macro-detail"]
    );
}

#[test]
fn approved_manifest_rejects_unknown_translation_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = translation("verified", "target", true, json!([]));
    item["evidence"] = json!(["evidence:missing"]);
    value["translations"] = json!([item]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["translation translation:lisp-to-ir references unknown evidence evidence:missing"]
    );
}

#[test]
fn approved_manifest_rejects_lossy_source_conformance_claim() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["translations"] = json!([translation("lossy", "source", false, json!([loss("L1")]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "lossy translation translation:lisp-to-ir cannot claim source conformance",
            "non-round-trip translation translation:lisp-to-ir cannot claim source conformance"
        ]
    );
}

fn translation(status: &str, conformance: &str, round_trip: bool, losses: Value) -> Value {
    json!({
        "id": "translation:lisp-to-ir",
        "source_context": "context:lisp-ssot",
        "target_context": "context:canonical-ir",
        "status": status,
        "conformance": conformance,
        "round_trip": round_trip,
        "interpreted_under": "0.1.0",
        "approved_by": "gate:quality",
        "evidence": ["evidence:quality"],
        "losses": losses
    })
}
fn loss(level: &str) -> Value {
    json!({
        "id": "loss:macro-detail",
        "level": level,
        "description": "Surface authoring form is normalized away.",
        "status": "accepted",
        "evidence": ["evidence:quality"]
    })
}
