use anyhow::Result;
use dslraid_core::{fsm_local_name, Fsm, State, Transition};
use std::fmt::Write;

use crate::escape::ident;

pub(super) fn write_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    writeln!(out, "digraph {} {{", ident(fsm_local_name(&fsm.id)))?;
    writeln!(out, "  rankdir=LR;")?;
    for state in &fsm.states {
        write_state(out, state)?;
    }
    for transition in &fsm.transitions {
        write_transition(out, transition)?;
    }
    writeln!(out, "}}")?;
    Ok(())
}

fn write_state(out: &mut String, state: &State) -> Result<()> {
    writeln!(
        out,
        "  {} [shape={} label=\"{}\"];",
        ident(&state.id),
        shape(state),
        state.id
    )?;
    Ok(())
}

fn write_transition(out: &mut String, transition: &Transition) -> Result<()> {
    writeln!(
        out,
        "  {} -> {} [label=\"{}\"];",
        ident(&transition.from),
        ident(&transition.to),
        transition.on.as_deref().unwrap_or("epsilon")
    )?;
    Ok(())
}

fn shape(state: &State) -> &'static str {
    if state.terminal {
        "doublecircle"
    } else {
        "box"
    }
}
