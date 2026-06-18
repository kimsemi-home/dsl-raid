use std::collections::BTreeMap;

pub(super) type CoverageCounters = BTreeMap<String, CoverageCounter>;

#[derive(Debug, Clone)]
pub(super) struct CoverageCounter {
    pub(super) kind: String,
    pub(super) count: usize,
    pub(super) failures: usize,
    pub(super) status_override: Option<String>,
    pub(super) last_seen: Option<String>,
}

impl CoverageCounter {
    pub(super) fn new(kind: &str) -> Self {
        Self {
            kind: kind.to_string(),
            count: 0,
            failures: 0,
            status_override: None,
            last_seen: None,
        }
    }
}

pub(super) fn mark_coverage(
    counters: &mut CoverageCounters,
    subject: &str,
    failed: bool,
    timestamp: Option<String>,
    status_override: Option<&str>,
) {
    if let Some(counter) = counters.get_mut(subject) {
        counter.count += 1;
        if failed {
            counter.failures += 1;
        }
        if let Some(status_override) = status_override {
            counter.status_override = Some(status_override.to_string());
        }
        if let Some(timestamp) = timestamp {
            counter.last_seen = Some(timestamp);
        }
    }
}
