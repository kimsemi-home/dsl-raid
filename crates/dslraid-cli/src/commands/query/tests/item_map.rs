use super::fixtures::{effect_fixture, runscope_fixture};
use dslraid_core::load_core_ir;
use serde_json::Value;

#[test]
fn item_map_exposes_transition_endpoints() {
    let ir = load_core_ir(runscope_fixture()).unwrap();
    let items = super::super::item_map(&ir);

    let transition = items
        .get("transition:runtime.idle_to_starting")
        .expect("fixture transition is indexed");

    assert_eq!(
        transition.get("from").and_then(Value::as_str),
        Some("state:runtime.idle")
    );
    assert_eq!(
        transition.get("to").and_then(Value::as_str),
        Some("state:runtime.starting")
    );
}

#[test]
fn item_map_exposes_guard_action_effects() {
    let ir = load_core_ir(effect_fixture()).unwrap();
    let items = super::super::item_map(&ir);
    let guard = items.get("guard:runtime.can_start").unwrap();
    let action = items.get("action:runtime.emit_started").unwrap();
    assert_eq!(guard["guard_kind"], "capability");
    assert_eq!(guard["expression"]["source"], "runtime-ready");
    assert_eq!(action["action_kind"], "emit");
    assert_eq!(action["command"], "command:runtime_start");
    let filtered = super::super::values(&ir, "kind=guard and expression.source~=ready").unwrap();
    assert_eq!(filtered[0]["subject"], "guard:runtime.can_start");
}
