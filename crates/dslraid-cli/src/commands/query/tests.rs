use super::*;
use std::path::PathBuf;

#[test]
fn values_support_policy_or_terminal_query() {
    let ir = load_core_ir(runscope_fixture()).unwrap();

    let results = values(
        &ir,
        "kind=transition and requires~=policy:no_secret_leak or terminal=true",
    )
    .unwrap();

    assert!(results.iter().any(|item| {
        item.get("subject").and_then(Value::as_str)
            == Some("transition:runtime.running_to_completed")
    }));
    assert!(results.iter().any(|item| {
        item.get("subject").and_then(Value::as_str) == Some("state:runtime.completed")
    }));
}

#[test]
fn values_support_numeric_query() {
    let ir = load_core_ir(runscope_fixture()).unwrap();

    let results = values(&ir, "kind=fsm and states>=1").unwrap();

    assert!(results
        .iter()
        .any(|item| item.get("subject").and_then(Value::as_str) == Some("fsm:runtime")));
}

#[test]
fn item_map_exposes_transition_endpoints() {
    let ir = load_core_ir(runscope_fixture()).unwrap();
    let items = item_map(&ir);

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
    let items = item_map(&ir);
    let guard = items.get("guard:runtime.can_start").unwrap();
    let action = items.get("action:runtime.emit_started").unwrap();
    assert_eq!(guard["guard_kind"], "capability");
    assert_eq!(guard["expression"]["source"], "runtime-ready");
    assert_eq!(action["action_kind"], "emit");
    assert_eq!(action["command"], "command:runtime_start");
    let filtered = values(&ir, "kind=guard and expression.source~=ready").unwrap();
    assert_eq!(filtered[0]["subject"], "guard:runtime.can_start");
}
fn runscope_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.raid.json")
}

fn effect_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/golden/query/guard-action.input.json")
}
