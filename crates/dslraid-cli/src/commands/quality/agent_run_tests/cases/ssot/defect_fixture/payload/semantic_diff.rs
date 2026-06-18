use serde_json::{json, Value};

pub(super) fn fixture() -> Value {
    json!({
        "id": "semantic-diff:ssot-defect",
        "base_hash": "sha256:base",
        "head_hash": "sha256:core",
        "status": "changed",
        "summary": "Canonical IR changed to patch the SSOT defect.",
        "evidence": ["evidence:quality", "evidence:trace"]
    })
}
