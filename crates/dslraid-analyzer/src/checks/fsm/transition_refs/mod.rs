mod collect;
mod event_record;
mod failures;
mod guard_action_record;
mod push;
mod record;
mod requires_record;
mod sets;
mod state_record;

use dslraid_core::{CoreIr, Fsm};

use crate::builder::ReportBuilder;

pub(crate) fn check(fsm: &Fsm, ir: &CoreIr, builder: &mut ReportBuilder) {
    let failures = collect::collect(fsm, ir);
    record::from_state(builder, &failures.unknown_from);
    record::to_state(builder, &failures.unknown_to);
    record::event(builder, &failures.unknown_events);
    record::guard_action(builder, &failures.unknown_guard_action);
    record::requires(builder, &failures.unknown_requires);
}
