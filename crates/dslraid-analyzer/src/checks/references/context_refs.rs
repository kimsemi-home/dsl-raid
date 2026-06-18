use dslraid_core::CoreIr;
use serde_json::Value;
use std::collections::BTreeSet;

use super::missing;

pub(super) fn collect(ir: &CoreIr, subjects: &BTreeSet<String>, missing: &mut Vec<Value>) {
    for context in &ir.contexts {
        for owned in &context.owns {
            missing::push_if_missing(subjects, missing, &context.id, owned);
        }
    }
}
