use anyhow::Result;
use dslraid_core::Fsm;
use std::fmt::Write;

use crate::names::go_type;

pub(super) fn write_consts(out: &mut String, fsm: &Fsm, type_name: &str) -> Result<()> {
    let width = const_width(fsm, type_name);
    writeln!(out, "const (")?;
    for state in &fsm.states {
        let name = const_name(type_name, &state.id);
        let padding = " ".repeat(width - name.len() + 1);
        writeln!(out, "\t{name}{padding}{type_name}State = \"{}\"", state.id)?;
    }
    writeln!(out, ")\n")?;
    Ok(())
}

fn const_width(fsm: &Fsm, type_name: &str) -> usize {
    fsm.states
        .iter()
        .map(|state| const_name(type_name, &state.id).len())
        .max()
        .unwrap_or_default()
}

fn const_name(type_name: &str, state: &str) -> String {
    format!("{type_name}State{}", go_type(state))
}
