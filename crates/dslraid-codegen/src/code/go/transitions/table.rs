use anyhow::Result;
use dslraid_core::Fsm;
use std::fmt::Write;

use crate::names::camel;

mod outgoing;
mod row;
mod width;

pub(super) fn name(fsm: &Fsm) -> String {
    format!("{}Transitions", camel(&fsm.name))
}

pub(super) fn write(out: &mut String, fsm: &Fsm, type_name: &str, table: &str) -> Result<()> {
    writeln!(
        out,
        "var {table} = map[{type_name}State]map[string]{type_name}State{{"
    )?;
    let width = width::row_width(fsm, type_name);
    for state in &fsm.states {
        row::write_row(out, fsm, type_name, &state.id, width)?;
    }
    writeln!(out, "}}\n")?;
    Ok(())
}
