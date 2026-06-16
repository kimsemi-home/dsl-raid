use super::render_markdown;
use super::report::report;
use dslraid_core::load_core_ir;
use std::path::{Path, PathBuf};

#[test]
fn diff_report_detects_added_untested_transition() {
    let base = load_core_ir(runscope_fixture()).unwrap();
    let mut head = base.clone();
    let fsm = head.fsms.first_mut().expect("RunScope fixture has an FSM");
    fsm.states.push(
        serde_json::from_value(serde_json::json!({
            "id": "retrying",
            "kind": "atomic"
        }))
        .unwrap(),
    );
    fsm.transitions.push(
        serde_json::from_value(serde_json::json!({
            "id": "running_to_retrying",
            "from": "running",
            "to": "retrying",
            "guards": [],
            "actions": []
        }))
        .unwrap(),
    );

    let report = report(&base, &head, Path::new("base.json"), Path::new("head.json")).unwrap();

    assert_eq!(report.status, "changed");
    assert_eq!(report.summary.review.states_added, 1);
    assert_eq!(report.summary.review.transitions_added, 1);
    assert_eq!(report.summary.review.untested_transitions_added, 1);
    assert!(report.changes.iter().any(|change| {
        change.action == "added" && change.subject == "transition:runtime.running_to_retrying"
    }));
    assert!(report
        .warnings
        .iter()
        .any(|warning| warning.code == "DIF010"));
}

#[test]
fn diff_markdown_renders_unchanged_summary() {
    let ir = load_core_ir(runscope_fixture()).unwrap();
    let report = report(&ir, &ir, Path::new("base.json"), Path::new("head.json")).unwrap();
    let markdown = render_markdown::render(&report);

    assert_eq!(report.status, "unchanged");
    assert!(markdown.contains("Status: **unchanged**"));
    assert!(markdown.contains("## Summary"));
    assert!(markdown.contains("- none"));
}

fn runscope_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.raid.json")
}
