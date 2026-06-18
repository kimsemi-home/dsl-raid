mod capacity;
mod claim;
mod quarantine;
mod semantic_diff;
mod steward;

use serde_json::Value;

pub(super) use steward::attach_steward_evidence;

pub(super) fn capacity() -> Value {
    capacity::fixture()
}

pub(super) fn claim(plan: Option<&str>) -> Value {
    claim::fixture(plan)
}

pub(super) fn quarantine() -> Value {
    quarantine::fixture()
}

pub(super) fn semantic_diff() -> Value {
    semantic_diff::fixture()
}
