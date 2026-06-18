use dslraid_core::CoreIr;
use serde_json::Value;
use std::collections::BTreeSet;

use super::missing;

pub(super) fn collect(ir: &CoreIr, subjects: &BTreeSet<String>, missing: &mut Vec<Value>) {
    for requirement in &ir.requirements {
        for subject in &requirement.satisfied_by {
            missing::push_if_missing(subjects, missing, &requirement.id, subject);
        }
    }
}
