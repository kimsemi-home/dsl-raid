use dslraid_core::load_core_ir;
use std::path::{Path, PathBuf};

use super::project_view;

#[test]
fn project_composition_view_materializes_tuple_scene() {
    let ir = load_core_ir(runscope_fixture()).unwrap();
    let view = project_view(&ir, Some("view:runscope"), "runscope.raid.json").unwrap();

    assert_eq!(view.layout.engine, "bounded-reachable-product");
    assert!(view
        .nodes
        .iter()
        .any(|node| node.subject.starts_with("state_tuple:")));
    assert!(view
        .edges
        .iter()
        .any(|edge| edge.subject.starts_with("tuple_transition:")));
    assert!(view
        .inspector_panels
        .iter()
        .any(|panel| panel.subject == "composition:runscope"));
}

fn runscope_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.raid.json")
}
