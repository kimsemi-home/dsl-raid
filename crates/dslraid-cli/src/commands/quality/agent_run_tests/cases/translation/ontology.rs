use super::super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn translation_requires_interpreted_under() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = translation("0.1.0");
    item.as_object_mut().unwrap().remove("interpreted_under");
    value["translations"] = json!([item]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["translation translation:lisp-to-ir requires interpreted_under"]
    );
}

#[test]
fn translation_interpreter_must_match_ssot_ontology() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["translations"] = json!([translation("9.9.9")]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["translation translation:lisp-to-ir interpreted_under 9.9.9 differs from ssot 0.1.0"]
    );
}

fn translation(interpreted_under: &str) -> Value {
    json!({
        "id": "translation:lisp-to-ir",
        "source_context": "context:lisp-ssot",
        "target_context": "context:canonical-ir",
        "status": "verified",
        "conformance": "target",
        "interpreted_under": interpreted_under,
        "approved_by": "gate:quality",
        "evidence": ["evidence:quality"],
        "losses": []
    })
}
