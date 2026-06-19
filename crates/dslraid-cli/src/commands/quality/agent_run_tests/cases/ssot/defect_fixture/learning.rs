use super::super::super::fixtures::adversarial;
use super::super::super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

pub(super) fn unlinked_learning_update() -> Value {
    mutated("evidence", json!(["evidence:trace"]))
}

pub(super) fn stale_learning_update() -> Value {
    mutated("ontology_version", json!("0.0.9"))
}

pub(super) fn unscoped_learning_update() -> Value {
    mutated("affected_subjects", json!(["agent-run:other"]))
}

pub(super) fn unlinked_prior_update() -> Value {
    mutated("supersedes", json!([]))
}

pub(super) fn unverified_learning_update() -> Value {
    mutated("verification_plan", json!("verification:other"))
}

pub(super) fn unowned_learning_update() -> Value {
    mutated("owner", json!("sidecar:dslraid-quality"))
}

fn mutated(field: &str, replacement: Value) -> Value {
    let mut value = base_manifest(adversarial(), "finished", high());
    super::governance::apply(&mut value);
    value["debts"][0]["updates"][0][field] = replacement;
    value
}
