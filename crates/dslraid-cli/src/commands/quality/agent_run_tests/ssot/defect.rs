use super::defect_fixture::{governed, routine};

#[test]
fn ssot_defect_claim_requires_governance_plan_freeze_and_diff() {
    let issues = super::super::super::agent_run::semantic_issues(&routine());
    let expected = [
        "ssot defect claim claim:ssot-defect requires authority governance scope",
        "ssot defect claim claim:ssot-defect requires verification plan",
        "ssot defect claim claim:ssot-defect requires quarantine containment record",
        "ssot defect claim claim:ssot-defect requires changed semantic diff",
        "ssot defect claim claim:ssot-defect requires changed semantic diff summary",
        "ssot defect claim claim:ssot-defect requires closed review debt",
    ];
    assert_eq!(issues, expected);
}

#[test]
fn governed_ssot_defect_accepts_released_quarantine_history() {
    assert_eq!(
        super::super::super::agent_run::semantic_issues(&governed()),
        Vec::<String>::new()
    );
}
