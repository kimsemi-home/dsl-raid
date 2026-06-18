use serde_json::{json, Value};

pub(super) fn evidence(evidence: &mut Value) -> Value {
    let Some(items) = evidence.as_array_mut() else {
        return json!([]);
    };
    if let Some(id) = items.iter().find_map(|item| item.get("id").cloned()) {
        return json!([id]);
    }
    let Some(first) = items.first_mut() else {
        return json!([]);
    };
    first["id"] = json!("evidence:authority");
    json!(["evidence:authority"])
}

pub(super) fn attach_producer_reliability(value: &mut Value) {
    add_authority_ref(value, "evidence:producer-codex");
    value["evidence"].as_array_mut().unwrap().push(json!({
        "id": "evidence:producer-codex",
        "kind": "decision",
        "uri": "evidence/producer-codex.json",
        "subject": "agent:codex",
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
