use dslraid_core::{CoreIr, Derivation};
use serde_json::{json, Value};

use super::{artifacts, sources};

pub(super) fn derivations(ir: &CoreIr) -> Vec<Value> {
    let known_sources = sources::ids(ir);
    let known_artifacts = artifacts::ids(ir);
    let mut failures = Vec::new();
    for derivation in &ir.derivations {
        if !known_sources.contains(&derivation.source) {
            failures.push(source_failure(derivation));
        }
        failures.extend(missing_artifacts(derivation, &known_artifacts));
    }
    failures
}

fn missing_artifacts(
    derivation: &Derivation,
    known_artifacts: &std::collections::BTreeSet<String>,
) -> Vec<Value> {
    derivation
        .targets
        .iter()
        .filter(|target| !known_artifacts.contains(&target.artifact))
        .map(|target| json!({ "derivation": derivation.id, "artifact": target.artifact }))
        .collect()
}

fn source_failure(derivation: &Derivation) -> Value {
    json!({ "derivation": derivation.id, "source": derivation.source })
}
