mod payload;
mod retrospective;

use super::super::fixtures::{base_manifest, high};
use super::super::fixtures_reviewer::adversarial;
use payload::{capacity, claim, quarantine, semantic_diff};
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
    let mut value = base_manifest(adversarial(), "finished", high());
    govern(&mut value);
    value["debts"][0]["updates"][0]["evidence"] = json!(["evidence:trace"]);
    value
}

fn govern(value: &mut Value) {
    value["producer"]["trust_tier"] = json!("T3");
    value["authority_gate"]["profile"] = json!("governance");
    value["authority_gate"]["scope"] = json!("authority");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("steward:ops");
    value["orchestration"]["authority_profile"] = json!("governance");
    value["review_capacity"] = capacity();
    value["semantic_diffs"] = json!([semantic_diff()]);
    value["containments"] = json!([quarantine()]);
    value["claims"] = json!([claim(Some("verification:quality"))]);
    value["debts"] = json!([review_debt()]);
}
