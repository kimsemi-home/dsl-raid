mod accountability;
mod evidence;
mod impact;
mod lineage;
mod selection;
mod verification;
mod version;

use crate::commands::quality::agent_run::fields::field_text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, claim: &Value, issues: &mut Vec<String>) {
    let claim_id = field_text(claim, "id").unwrap_or("<unknown>");
    let Some(update) = selection::linked_update(value, claim) else {
        issues.push(format!(
            "ssot defect claim {} requires linked knowledge update",
            claim_id
        ));
        return;
    };
    if !version::matches(value, update) {
        push(claim_id, "current ontology knowledge update", issues);
    }
    if !impact::covers(update, claim) {
        push(claim_id, "affected knowledge update subject", issues);
    }
    if !lineage::has_prior(update) {
        push(claim_id, "prior knowledge link", issues);
    }
    if !verification::matches(update, claim) {
        push(claim_id, "knowledge update verification plan", issues);
    }
    if !accountability::matches(value, update) {
        push(claim_id, "authority knowledge update owner", issues);
    }
}

fn push(claim_id: &str, requirement: &str, issues: &mut Vec<String>) {
    issues.push(format!(
        "ssot defect claim {claim_id} requires {requirement}"
    ));
}
