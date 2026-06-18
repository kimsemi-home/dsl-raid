mod learning;
mod payload;
mod retrospective;

use super::super::fixtures::{base_manifest, high};
use super::super::fixtures_reviewer::adversarial;
use payload::{attach_steward_evidence, capacity, claim, quarantine, semantic_diff};
use retrospective::review_debt;
use serde_json::{json, Value};

pub(super) fn routine() -> Value {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([claim(None)]);
    value
}

pub(super) fn governed() -> Value {
    let mut value = base_manifest(adversarial(), "finished", high());
    govern(&mut value);
    value
}

pub(super) fn unlinked_retrospective() -> Value {
    let mut value = base_manifest(adversarial(), "finished", high());
    govern(&mut value);
    value["debts"][0]["evidence"] = json!(["evidence:trace"]);
    value
}

pub(super) fn unlinked_learning_update() -> Value {
    learning::unlinked_learning_update()
}

pub(super) fn stale_learning_update() -> Value {
    learning::stale_learning_update()
}

pub(super) fn unscoped_learning_update() -> Value {
    learning::unscoped_learning_update()
}

pub(super) fn unlinked_prior_update() -> Value {
    learning::unlinked_prior_update()
}

pub(super) fn unverified_learning_update() -> Value {
    learning::unverified_learning_update()
}

pub(super) fn unowned_learning_update() -> Value {
    learning::unowned_learning_update()
}

fn govern(value: &mut Value) {
    value["producer"]["trust_tier"] = json!("T3");
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
