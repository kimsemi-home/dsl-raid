use dslraid_core::CoreIr;
use serde_json::{json, Value};

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(crate) fn check(ir: &CoreIr, builder: &mut ReportBuilder) {
    let leaked = secret_artifacts(ir);
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V046",
            assertion: "assertion:security.public_projection_no_secret",
            code: "SEC046",
            layer: "visibility_security",
            predicate: "public_projection_no_secret",
            severity: "error",
            failures: &leaked,
            pass_message: "Public IR fixture has no secret-bearing artifacts.",
            fail_message: "Secret-bearing artifacts are visible in a public projection candidate.",
            suggestion: "Remove the secret artifact or mark it private and exclude it from public projections.",
        },
    );
}

fn secret_artifacts(ir: &CoreIr) -> Vec<Value> {
    ir.artifacts
        .iter()
        .filter(|artifact| artifact.visibility.as_deref() == Some("secret"))
        .map(|artifact| json!({ "artifact": artifact.id, "path": artifact.path }))
        .collect()
}
