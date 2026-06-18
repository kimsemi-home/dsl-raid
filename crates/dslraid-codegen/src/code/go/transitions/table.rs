use anyhow::Result;
use dslraid_core::{Fsm, Transition};
use std::fmt::Write;

use crate::names::{camel, go_type};

pub(super) fn name(fsm: &Fsm) -> String {
    format!("{}Transitions", camel(&fsm.name))
}

pub(super) fn write(out: &mut String, fsm: &Fsm, type_name: &str, table: &str) -> Result<()> {
    writeln!(
        out,
        "var {table} = map[{type_name}State]map[string]{type_name}State{{"
    )?;
    let width = row_width(fsm, type_name);
    for state in &fsm.states {
        write_row(out, fsm, type_name, &state.id, width)?;
    }
    writeln!(out, "}}\n")?;
    Ok(())
}

fn write_row(
    out: &mut String,
    fsm: &Fsm,
    type_name: &str,
    state: &str,
    width: usize,
) -> Result<()> {
    let transitions = outgoing(fsm, state);
    if transitions.is_empty() {
        return Ok(());
    }
    let key = format!("{type_name}State{}", go_type(state));
    let padding = " ".repeat(width - key.len() + 1);
    write!(out, "\t{key}:{padding}{{")?;
    for (index, transition) in transitions.iter().enumerate() {
        if index > 0 {
            write!(out, ", ")?;
        }
        write_entry(out, transition, type_name)?;
    }
    writeln!(out, "}},")?;
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

fn row_width(fsm: &Fsm, type_name: &str) -> usize {
    fsm.states
        .iter()
        .filter(|state| !outgoing(fsm, &state.id).is_empty())
        .map(|state| format!("{type_name}State{}", go_type(&state.id)).len())
        .max()
        .unwrap_or_default()
}

fn outgoing<'a>(fsm: &'a Fsm, state: &str) -> Vec<&'a Transition> {
    fsm.transitions
        .iter()
        .filter(|transition| transition.from == state)
        .collect()
}
