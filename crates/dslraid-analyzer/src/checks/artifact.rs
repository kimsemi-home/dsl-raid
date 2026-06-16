use std::collections::BTreeSet;

use dslraid_core::CoreIr;
use serde_json::{json, Value};

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(crate) fn check(ir: &CoreIr, builder: &mut ReportBuilder) {
    let orphan_generated = orphan_generated(ir);
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V036",
            assertion: "assertion:artifact.generated_by_exists",
            code: "ART036",
            layer: "artifact",
            predicate: "generated_artifact_has_derivation",
            severity: "warning",
            failures: &orphan_generated,
            pass_message: "Generated artifacts have derivation provenance.",
            fail_message: "Some generated artifacts do not trace to a derivation.",
            suggestion: "Set generated_by to an existing derivation ID.",
        },
    );
}

fn orphan_generated(ir: &CoreIr) -> Vec<Value> {
    let derivations: BTreeSet<_> = ir
        .derivations
        .iter()
        .map(|derivation| derivation.id.as_str())
        .collect();
    ir.artifacts
        .iter()
        .filter(|artifact| artifact.kind == "generated")
        .filter(|artifact| match artifact.generated_by.as_deref() {
            Some(id) => !derivations.contains(id),
            None => true,
        })
        .map(|artifact| json!({ "artifact": artifact.id, "generated_by": artifact.generated_by }))
        .collect()
}
