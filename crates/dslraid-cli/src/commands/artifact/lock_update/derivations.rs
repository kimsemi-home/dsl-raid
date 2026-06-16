use dslraid_core::{CoreIr, Derivation};

use super::model::DerivationRecord;

pub(super) fn records(ir: &CoreIr, current_hash: &str) -> Vec<DerivationRecord> {
    ir.derivations
        .iter()
        .map(|derivation| record(derivation, current_hash))
        .collect()
}

fn record(derivation: &Derivation, current_hash: &str) -> DerivationRecord {
    DerivationRecord {
        derivation: derivation.id.clone(),
        rule: derivation.rule.id.clone(),
        generator: super::tool::named(
            derivation.rule.generator.as_deref().unwrap_or("dslraid"),
            derivation.rule.version.as_deref().unwrap_or("0.1.0"),
        ),
        input_hash: current_hash.to_string(),
        targets: derivation
            .targets
            .iter()
            .map(|target| target.artifact.clone())
            .collect(),
    }
}
