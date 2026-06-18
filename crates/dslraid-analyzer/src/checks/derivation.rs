mod artifacts;
mod broken;
mod sources;

use dslraid_core::CoreIr;

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(crate) fn check(ir: &CoreIr, builder: &mut ReportBuilder) {
    let broken = broken::derivations(ir);
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
