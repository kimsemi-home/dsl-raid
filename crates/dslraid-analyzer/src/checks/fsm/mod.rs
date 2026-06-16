mod determinism;
mod initial;
mod reachability;
mod source;
mod state_count;
mod terminal;
mod transition_refs;

use dslraid_core::{CoreIr, Fsm};

use crate::builder::ReportBuilder;

pub(crate) fn check(fsm: &Fsm, ir: &CoreIr, builder: &mut ReportBuilder) {
    state_count::check(fsm, builder);
    initial::check(fsm, builder);
    transition_refs::check(fsm, ir, builder);
    terminal::check(fsm, builder);
    reachability::check(fsm, builder);
    determinism::check(fsm, builder);
    source::check(fsm, builder);
}
