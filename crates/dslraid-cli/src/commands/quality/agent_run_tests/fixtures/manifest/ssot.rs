use serde_json::{json, Value};

pub(super) fn fixture() -> Value {
    json!({
        "context": "runscope",
        "core_ir": "examples/runscope/runscope.raid.json",
        "core_ir_hash": "sha256:core",
        "ontology_version": "0.1.0",
        "contract_version": "0.1.0",
        "revalidation": {
            "status": "valid",
            "assessed_at": "2026-06-17T00:00:00Z",
            "assessor": "sidecar:dslraid-quality",
            "revalidate_at": "2026-07-17T00:00:00Z"
        }
    })
}
