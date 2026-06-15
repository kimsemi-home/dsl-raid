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

fn runscope_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.raid.json")
}
