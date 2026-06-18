use serde_json::Value;

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(super) fn record(builder: &mut ReportBuilder, failures: &[Value]) {
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V017",
            assertion: "assertion:guard.references_existing_capability",
            code: "GUA017",
            layer: "guard_action",
            predicate: "guard_references_existing_capability",
            severity: "error",
            failures,
            pass_message: "All guard/action references resolve inside their FSM.",
            fail_message: "Some guard/action references do not resolve inside their FSM.",
            suggestion: "Declare the guard or action before referencing it.",
        },
    );
}
