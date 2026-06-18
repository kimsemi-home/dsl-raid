use anyhow::Result;
use dslraid_core::{Fsm, Transition};
use std::fmt::Write;

use crate::names::go_type;

pub(super) fn write_row(
    out: &mut String,
    fsm: &Fsm,
    type_name: &str,
    state: &str,
    width: usize,
) -> Result<()> {
    let transitions = super::outgoing::outgoing(fsm, state);
    if transitions.is_empty() {
        return Ok(());
    }
    let key = format!("{type_name}State{}", go_type(state));
    let padding = " ".repeat(width - key.len() + 1);
    write!(out, "\t{key}:{padding}{{")?;
    write_entries(out, &transitions, type_name)?;
    writeln!(out, "}},")?;
    Ok(())
}

fn write_entries(out: &mut String, transitions: &[&Transition], type_name: &str) -> Result<()> {
    for (index, transition) in transitions.iter().enumerate() {
        if index > 0 {
            write!(out, ", ")?;
        }
        write_entry(out, transition, type_name)?;
    }
    Ok(())
}

fn write_entry(out: &mut String, transition: &Transition, type_name: &str) -> Result<()> {
    let event = transition.on.as_deref().unwrap_or("");
    write!(
        out,
        "\"{event}\": {type_name}State{}",
        go_type(&transition.to)
    )?;
    Ok(())
}
