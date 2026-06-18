use serde_json::{json, Value};

pub(crate) fn attach_steward_evidence(value: &mut Value) {
    add_authority_ref(value, "evidence:steward-ops");
    value["evidence"].as_array_mut().unwrap().push(json!({
        "id": "evidence:steward-ops",
        "kind": "decision",
        "uri": "evidence/steward-ops.json",
        "subject": "steward:ops",
        "provenance": {
            "kind": "human-annotation",
            "observed_by": "human:alice",
            "observed_at": "2026-06-17T00:00:00Z",
            "ontology_version": "0.1.0"
        }
    }));
}

fn add_authority_ref(value: &mut Value, id: &str) {
    let refs = value["authority_gate"]["evidence"].as_array_mut().unwrap();
    if !refs.iter().any(|item| item.as_str() == Some(id)) {
        refs.push(json!(id));
    }
}
