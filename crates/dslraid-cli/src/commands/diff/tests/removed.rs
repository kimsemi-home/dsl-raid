use super::{report, runscope_fixture};
use dslraid_core::load_core_ir;
use std::path::Path;

#[test]
fn diff_report_detects_removed_transition() {
    let base = load_core_ir(runscope_fixture()).unwrap();
    let mut head = base.clone();
    let fsm = head.fsms.first_mut().expect("RunScope fixture has an FSM");
    fsm.transitions
        .retain(|transition| transition.id != "running_to_completed");

    let report = report(&base, &head, Path::new("base.json"), Path::new("head.json")).unwrap();

    assert_eq!(report.status, "changed");
    assert_eq!(report.summary.review.transitions_removed, 1);
    assert!(report.changes.iter().any(|change| {
        change.action == "removed" && change.subject == "transition:runtime.running_to_completed"
    }));
}
