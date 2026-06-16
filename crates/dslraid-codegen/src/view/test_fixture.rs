use dslraid_core::CoreIr;

pub(crate) fn diagnostic_fixture() -> CoreIr {
    serde_json::from_value(serde_json::json!({
        "ir_version": "0.1.0",
        "project": {"id": "fixture", "name": "fixture"},
        "fsms": [{
            "id": "fsm:runtime",
            "name": "Runtime",
            "states": [
                {"id": "idle", "kind": "atomic", "initial": true},
                {"id": "running", "kind": "atomic"}
            ],
            "transitions": [{"id": "finish", "from": "running", "to": "idle"}]
        }],
        "projections": [{
            "id": "view:runtime",
            "kind": "projection",
            "source": "fsm:runtime"
        }],
        "diagnostics": [{
            "id": "diag:state",
            "code": "FSM999",
            "severity": "error",
            "message": "state fixture",
            "subjects": ["state:runtime.running"],
            "suggestion": "fix state"
        }, {
            "id": "diag:transition",
            "code": "FSM998",
            "severity": "warning",
            "message": "transition fixture",
            "subjects": ["transition:runtime.finish"]
        }]
    }))
    .unwrap()
}
