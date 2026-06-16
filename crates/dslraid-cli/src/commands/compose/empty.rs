use serde_json::Value;

pub(super) fn empty_result(
    materialize: &str,
    limit: usize,
    focus: Option<&str>,
    depth: usize,
) -> Value {
    serde_json::json!({
        "composition_version": "0.1.0",
        "composition": {
            "id": null,
            "name": null,
            "kind": null,
            "inputs": [],
            "mode": materialize,
            "state_space": 0,
            "limit": limit,
            "lazy": true,
            "truncated": false,
            "focus": focus,
            "depth": depth
        },
        "states": [],
        "transitions": [],
        "diagnostics": [{
            "code": "CMP000",
            "severity": "info",
            "message": "No compositions defined; nothing to compose.",
            "subjects": []
        }]
    })
}
