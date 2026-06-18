use super::super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn historical_evidence_accepts_translation_bridge() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["provenance"]["ontology_version"] = json!("0.0.9");
    value["translations"] = json!([translation()]);

    assert_eq!(
        super::super::super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

fn translation() -> serde_json::Value {
    json!({
        "id": "translation:historical-evidence",
        "source_context": "context:legacy-run",
        "target_context": "runscope",
        "status": "verified",
        "conformance": "target",
        "interpreted_under": "0.1.0",
        "approved_by": "gate:quality",
        "evidence": ["evidence:quality"],
        "losses": []
    })
}
