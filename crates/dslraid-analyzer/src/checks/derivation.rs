use std::collections::BTreeSet;

use dslraid_core::CoreIr;
use serde_json::{json, Value};

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(crate) fn check(ir: &CoreIr, builder: &mut ReportBuilder) {
    let broken = broken_derivations(ir);
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V034",
            assertion: "assertion:traceability.generated_artifact_traced",
            code: "TRC034",
            layer: "traceability",
            predicate: "generated_artifact_traced",
            severity: "error",
            failures: &broken,
            pass_message: "Generated artifacts trace back to derivations.",
            fail_message: "A derivation references a missing source or artifact.",
            suggestion: "Add the missing source/artifact or update the derivation target.",
        },
    );
}

fn broken_derivations(ir: &CoreIr) -> Vec<Value> {
    let known_sources = known_sources(ir);
    let artifacts: BTreeSet<_> = ir
        .artifacts
        .iter()
        .map(|artifact| artifact.id.as_str())
        .collect();
    let mut broken = Vec::new();
    for derivation in &ir.derivations {
        if !known_sources.contains(&derivation.source) {
            broken.push(json!({ "derivation": derivation.id, "source": derivation.source }));
        }
        for target in &derivation.targets {
            if !artifacts.contains(target.artifact.as_str()) {
                broken.push(json!({ "derivation": derivation.id, "artifact": target.artifact }));
            }
        }
    }
    broken
}

fn known_sources(ir: &CoreIr) -> BTreeSet<String> {
    ir.fsms
        .iter()
        .map(|fsm| fsm.id.clone())
        .chain(
            ir.compositions
                .iter()
                .map(|composition| composition.id.clone()),
        )
        .chain(ir.contexts.iter().map(|context| context.id.clone()))
        .collect()
}
