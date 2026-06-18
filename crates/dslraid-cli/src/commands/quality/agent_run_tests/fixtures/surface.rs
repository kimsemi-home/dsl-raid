use serde_json::Value;

pub(in super::super) fn base_manifest(reviewers: Value, lease: &str, evidence: Value) -> Value {
    super::manifest::base_manifest(reviewers, lease, evidence)
}

pub(in super::super) fn adversarial() -> Value {
    super::reviewer::adversarial()
}

pub(in super::super) fn attach_producer_reliability(value: &mut Value) {
    super::authority::attach_producer_reliability(value);
}

pub(in super::super) fn fresh_lock() -> Value {
    super::evidence::fresh_lock()
}

pub(in super::super) fn high() -> Value {
    super::evidence::high()
}

pub(in super::super) fn high_snapshot() -> Value {
    super::evidence::high_snapshot()
}

pub(in super::super) fn push_pruned_extra(value: &mut Value) {
    super::pruning::push_pruned_extra(value);
}

pub(in super::super) fn tombstone() -> Value {
    super::pruning::tombstone()
}
