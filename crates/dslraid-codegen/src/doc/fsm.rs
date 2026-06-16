use dslraid_core::{Fsm, State, Transition};

use super::table;

pub(super) fn write(out: &mut String, fsm: &Fsm) {
    out.push_str("## FSM: ");
    out.push_str(&fsm.name);
    out.push_str("\n\n");
    write_states(out, fsm);
    write_events(out, fsm);
    write_transitions(out, fsm);
}

fn write_states(out: &mut String, fsm: &Fsm) {
    out.push_str("### States\n\n");
    table::header(out, &["State", "Initial", "Terminal", "Tags"]);
    for state in &fsm.states {
        table::row(out, &state_row(state));
    }
    out.push('\n');
}

fn write_events(out: &mut String, fsm: &Fsm) {
    out.push_str("### Events\n\n");
    table::header(out, &["Event", "Kind"]);
    for event in &fsm.events {
        table::row(
            out,
            &[event.id.clone(), event.kind.clone().unwrap_or_default()],
        );
    }
    out.push('\n');
}

fn write_transitions(out: &mut String, fsm: &Fsm) {
    out.push_str("### Transitions\n\n");
    table::header(out, &["Transition", "From", "To", "Event", "Requires"]);
    for transition in &fsm.transitions {
        table::row(out, &transition_row(transition));
    }
    out.push('\n');
}

fn state_row(state: &State) -> [String; 4] {
    [
        state.id.clone(),
        state.initial.to_string(),
        state.terminal.to_string(),
        state.tags.join(", "),
    ]
}

fn transition_row(transition: &Transition) -> [String; 5] {
    [
        transition.id.clone(),
        transition.from.clone(),
        transition.to.clone(),
        transition
            .on
            .clone()
            .unwrap_or_else(|| "epsilon".to_string()),
        transition.requires.join(", "),
    ]
}
