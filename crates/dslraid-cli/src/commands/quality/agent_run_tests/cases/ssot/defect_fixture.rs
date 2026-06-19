mod governance;
mod learning;
mod payload;
mod retrospective;

use super::super::fixtures::adversarial;
use super::super::fixtures::{base_manifest, high};
use payload::claim;
use serde_json::{json, Value};

pub(super) fn routine() -> Value {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([claim(None)]);
    value
}

pub(super) fn governed() -> Value {
    let mut value = base_manifest(adversarial(), "finished", high());
    governance::apply(&mut value);
    value
}

pub(super) fn unlinked_retrospective() -> Value {
    let mut value = base_manifest(adversarial(), "finished", high());
    governance::apply(&mut value);
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
