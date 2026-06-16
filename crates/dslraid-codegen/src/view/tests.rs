use dslraid_core::load_core_ir;
use std::path::{Path, PathBuf};

use super::project_view;
use super::test_fixture::diagnostic_fixture;

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

#[test]
fn project_fsm_view_marks_diagnostic_subjects() {
    let ir = diagnostic_fixture();
    let view = project_view(&ir, Some("view:runtime"), "fixture").unwrap();
    let running = view
        .nodes
        .iter()
        .find(|node| node.subject == "state:runtime.running")
        .unwrap();
    let panel = view
        .inspector_panels
        .iter()
        .find(|panel| panel.subject == "state:runtime.running")
        .unwrap();
    let finish = view
        .edges
        .iter()
        .find(|edge| edge.subject == "transition:runtime.finish")
        .unwrap();

    assert!(running.badges.contains(&"diag:error".to_string()));
    assert_eq!(running.style.as_ref().unwrap().tone, "danger");
    assert_eq!(finish.style.as_ref().unwrap().tone, "warning");
    assert!(panel.sections.iter().any(has_diagnostic_suggestion));
}

fn has_diagnostic_suggestion(section: &super::InspectorSection) -> bool {
    section.title == "Diagnostics"
        && section
            .rows
            .iter()
            .any(|row| row.label == "Suggestion" && row.value == "fix state")
}

fn runscope_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.raid.json")
}
