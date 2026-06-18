use serde_json::Value;

use crate::builder::ReportBuilder;
use crate::checks::{record_collection_check, CollectionCheck};

pub(super) fn record(builder: &mut ReportBuilder, failures: &[Value]) {
    record_collection_check(
        builder,
        CollectionCheck {
            proposition: "V018",
            assertion: "assertion:action.uses_allowed_capability",
            code: "ACT018",
            layer: "guard_action",
            predicate: "action_uses_allowed_capability",
            severity: "error",
            failures,
            pass_message: "All transition requirements resolve to semantic subjects.",
            fail_message: "Some transition requirements do not resolve.",
            suggestion:
                "Reference an existing policy, capability, constraint, or semantic subject.",
        },
    );
}
