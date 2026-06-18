use dslraid_core::CoreIr;
use serde_json::Value;
use std::collections::BTreeSet;

use super::missing;

pub(super) fn collect(ir: &CoreIr, subjects: &BTreeSet<String>, missing: &mut Vec<Value>) {
    for policy in &ir.policies {
        for subject in &policy.applies_to {
            missing::push_if_missing(subjects, missing, &policy.id, subject);
        }
    }
}
