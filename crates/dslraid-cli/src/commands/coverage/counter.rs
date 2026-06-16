use dslraid_core::{event_subject, state_subject, transition_subject, CoreIr};
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

pub(super) fn base_coverage_counters(ir: &CoreIr) -> CoverageCounters {
    let mut counters = BTreeMap::new();
    for fsm in &ir.fsms {
        for state in &fsm.states {
            counters.insert(
                state_subject(&fsm.id, &state.id),
                CoverageCounter::new("state"),
            );
        }
        for event in &fsm.events {
            counters.insert(
                event_subject(&fsm.id, &event.id),
                CoverageCounter::new("event"),
            );
        }
        for guard in &fsm.guards {
            counters.insert(
                format!("guard:{}.{}", fsm.local_name(), guard.id),
                CoverageCounter::new("guard"),
            );
        }
        for action in &fsm.actions {
            counters.insert(
                format!("action:{}.{}", fsm.local_name(), action.id),
                CoverageCounter::new("action"),
            );
        }
        for transition in &fsm.transitions {
            counters.insert(
                transition_subject(&fsm.id, &transition.id),
                CoverageCounter::new("transition"),
            );
        }
    }
    for artifact in &ir.artifacts {
        counters.insert(artifact.id.clone(), CoverageCounter::new("artifact"));
    }
    counters
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
