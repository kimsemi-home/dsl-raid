use super::defect_fixture::{
    stale_learning_update, unlinked_learning_update, unlinked_prior_update,
    unowned_learning_update, unscoped_learning_update, unverified_learning_update,
};

#[test]
fn ssot_defect_learning_update_must_link_claim_evidence() {
    expect_issue(
        &unlinked_learning_update(),
        "ssot defect claim claim:ssot-defect requires linked knowledge update",
    );
}

#[test]
fn ssot_defect_learning_update_must_match_current_ontology() {
    expect_issue(
        &stale_learning_update(),
        "ssot defect claim claim:ssot-defect requires current ontology knowledge update",
    );
}

#[test]
fn ssot_defect_learning_update_must_name_affected_subject() {
    expect_issue(
        &unscoped_learning_update(),
        "ssot defect claim claim:ssot-defect requires affected knowledge update subject",
    );
}

#[test]
fn ssot_defect_learning_update_must_link_prior_knowledge() {
    expect_issue(
        &unlinked_prior_update(),
        "ssot defect claim claim:ssot-defect requires prior knowledge link",
    );
}

#[test]
fn ssot_defect_learning_update_must_link_verification_plan() {
    expect_issue(
        &unverified_learning_update(),
        "ssot defect claim claim:ssot-defect requires knowledge update verification plan",
    );
}

#[test]
fn ssot_defect_learning_update_must_match_authority_owner() {
    expect_issue(
        &unowned_learning_update(),
        "ssot defect claim claim:ssot-defect requires authority knowledge update owner",
    );
}

fn expect_issue(value: &serde_json::Value, issue: &str) {
    assert_eq!(
        super::super::super::agent_run::semantic_issues(value),
        vec![issue]
    );
}
