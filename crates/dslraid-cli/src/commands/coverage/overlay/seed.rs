use super::counter::{CoverageCounter, CoverageCounters};
use dslraid_core::{event_subject, state_subject, transition_subject, CoreIr};
use std::collections::BTreeMap;

pub(super) fn base_coverage_counters(ir: &CoreIr) -> CoverageCounters {
    let mut counters = BTreeMap::new();
    for fsm in &ir.fsms {
        seed_fsm_subjects(&mut counters, fsm);
    }
    for artifact in &ir.artifacts {
        counters.insert(artifact.id.clone(), CoverageCounter::new("artifact"));
    }
    counters
}

fn seed_fsm_subjects(counters: &mut CoverageCounters, fsm: &dslraid_core::Fsm) {
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
