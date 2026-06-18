use super::super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn lossy_translation_requires_approver() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["translations"] = json!([translation("lossy", json!(null))]);
    value["translations"][0]
        .as_object_mut()
        .unwrap()
        .remove("approved_by");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["translation translation:lisp-to-ir requires approver"]
    );
}

#[test]
fn translation_cannot_be_self_approved() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["translations"] = json!([translation("verified", json!("agent:codex"))]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["translation translation:lisp-to-ir cannot be self-approved"]
    );
}

fn translation(status: &str, approver: Value) -> Value {
    json!({
        "id": "translation:lisp-to-ir",
        "source_context": "context:lisp-ssot",
        "target_context": "context:canonical-ir",
        "status": status,
        "conformance": "target",
        "interpreted_under": "0.1.0",
        "approved_by": approver,
        "evidence": ["evidence:quality"],
        "losses": [{
            "id": "loss:lisp-authoring",
            "level": "L1",
            "description": "Authoring form is normalized.",
            "status": "accepted",
            "evidence": ["evidence:quality"]
        }]
    })
}
