use std::collections::BTreeSet;

use dslraid_core::CoreIr;
use serde_json::{json, Value};

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(crate) fn check(ir: &CoreIr, builder: &mut ReportBuilder) {
    let missing = missing_roots(ir);
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V029",
            assertion: "assertion:projection.root_exists",
            code: "PRJ029",
            layer: "projection",
            predicate: "projection_root_exists",
            severity: "error",
            failures: &missing,
            pass_message: "Projection roots resolve.",
            fail_message: "A projection root does not resolve.",
            suggestion: "Point projection.source at an existing fsm, composition, or context.",
        },
    );
}

fn missing_roots(ir: &CoreIr) -> Vec<Value> {
    let allowed = allowed_roots(ir);
    ir.projections
        .iter()
        .filter(|projection| !allowed.contains(&projection.source))
        .map(|projection| json!({ "projection": projection.id, "source": projection.source }))
        .collect()
}

fn allowed_roots(ir: &CoreIr) -> BTreeSet<String> {
    let mut allowed: BTreeSet<String> = ir.fsms.iter().map(|fsm| fsm.id.clone()).collect();
    allowed.extend(
        ir.compositions
            .iter()
            .map(|composition| composition.id.clone()),
    );
    allowed.extend(ir.contexts.iter().map(|context| context.id.clone()));
    allowed
}
