use super::super::super::fixtures::attach_producer_reliability;
use super::payload::{attach_steward_evidence, capacity, claim, quarantine, semantic_diff};
use super::retrospective::review_debt;
use serde_json::{json, Value};

pub(super) fn apply(value: &mut Value) {
    value["producer"]["trust_tier"] = json!("T3");
    attach_producer_reliability(value);
    value["authority_gate"]["profile"] = json!("governance");
    value["authority_gate"]["scope"] = json!("authority");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("steward:ops");
    attach_steward_evidence(value);
    value["orchestration"]["authority_profile"] = json!("governance");
    value["review_capacity"] = capacity();
    value["semantic_diffs"] = json!([semantic_diff()]);
    value["containments"] = json!([quarantine()]);
    value["claims"] = json!([claim(Some("verification:quality"))]);
    value["debts"] = json!([review_debt()]);
}
