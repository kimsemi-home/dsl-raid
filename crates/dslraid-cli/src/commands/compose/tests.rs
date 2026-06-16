use super::result;
use dslraid_core::load_core_ir;
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[test]
fn compose_diagnostics_only_keeps_materialization_lazy() {
    let ir = load_core_ir(runscope_fixture()).unwrap();

    let value = result(&ir, None, "diagnostics-only", 100, None, 1).unwrap();

    assert_eq!(
        value.get("states").and_then(Value::as_array).map(Vec::len),
        Some(0)
    );
    assert_eq!(
        value
            .get("composition")
            .and_then(|composition| composition.get("lazy"))
            .and_then(Value::as_bool),
        Some(true)
    );
}

#[test]
fn compose_reachable_materializes_fixture_state_space() {
    let ir = load_core_ir(runscope_fixture()).unwrap();

    let value = result(&ir, None, "reachable", 100, None, 1).unwrap();

    assert!(
        value
            .get("composition")
            .and_then(|composition| composition.get("state_space"))
            .and_then(Value::as_u64)
            .unwrap_or_default()
            > 0
    );
    assert!(!value
        .get("states")
        .and_then(Value::as_array)
        .unwrap()
        .is_empty());
}

#[test]
fn compose_reachable_transition_ids_are_unique() {
    let ir = load_core_ir(runscope_fixture()).unwrap();

    let value = result(&ir, None, "reachable", 100, None, 1).unwrap();
    let transitions = value.get("transitions").and_then(Value::as_array).unwrap();
    let ids = transitions
        .iter()
        .filter_map(|transition| transition.get("id").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();

    assert_eq!(ids.len(), transitions.len());
}

fn runscope_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.raid.json")
}
