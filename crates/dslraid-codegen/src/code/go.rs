use anyhow::Result;
use dslraid_core::{Fsm, Transition};
use std::fmt::Write;

use crate::names::go_type;

pub(super) fn write_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    let type_name = go_type(&fsm.name);
    write_type(out, &type_name)?;
    write_consts(out, fsm, &type_name)?;
    write_transition(out, fsm, &type_name)?;
    Ok(())
}

fn write_type(out: &mut String, type_name: &str) -> Result<()> {
    writeln!(out, "type {type_name}State string\n")?;
    Ok(())
}

fn write_consts(out: &mut String, fsm: &Fsm, type_name: &str) -> Result<()> {
    writeln!(out, "const (")?;
    for state in &fsm.states {
        writeln!(
            out,
            "\t{type_name}State{} {type_name}State = \"{}\"",
            go_type(&state.id),
            state.id
        )?;
    }
    writeln!(out, ")\n")?;
    Ok(())
}

fn write_transition(out: &mut String, fsm: &Fsm, type_name: &str) -> Result<()> {
    writeln!(
        out,
        "func {type_name}Transition(state {type_name}State, event string) ({type_name}State, bool) {{"
    )?;
    writeln!(out, "\tswitch state {{")?;
    for transition in &fsm.transitions {
        write_case(out, transition, type_name)?;
    }
    writeln!(out, "\t}}")?;
    writeln!(out, "\treturn state, false")?;
    writeln!(out, "}}\n")?;
    Ok(())
}

fn write_case(out: &mut String, transition: &Transition, type_name: &str) -> Result<()> {
    writeln!(out, "\tcase {type_name}State{}:", go_type(&transition.from))?;
    writeln!(
        out,
        "\t\tif event == \"{}\" {{ return {type_name}State{}, true }}",
        transition.on.as_deref().unwrap_or(""),
        go_type(&transition.to)
    )?;
    Ok(())
}
