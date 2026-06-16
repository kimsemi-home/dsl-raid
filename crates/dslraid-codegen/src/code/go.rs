mod consts;

use anyhow::Result;
use dslraid_core::{Fsm, Transition};
use std::fmt::Write;

use crate::names::go_type;

pub(super) fn write_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    let type_name = go_type(&fsm.name);
    write_type(out, &type_name)?;
    consts::write_consts(out, fsm, &type_name)?;
    write_transition(out, fsm, &type_name)?;
    Ok(())
}

fn write_type(out: &mut String, type_name: &str) -> Result<()> {
    writeln!(out, "type {type_name}State string\n")?;
    Ok(())
}

fn write_transition(out: &mut String, fsm: &Fsm, type_name: &str) -> Result<()> {
    writeln!(
        out,
        "func {type_name}Transition(state {type_name}State, event string) ({type_name}State, bool) {{"
    )?;
    writeln!(out, "\tswitch state {{")?;
    for state in &fsm.states {
        write_case(out, fsm, type_name, &state.id)?;
    }
    writeln!(out, "\t}}")?;
    writeln!(out, "\treturn state, false")?;
    writeln!(out, "}}")?;
    Ok(())
}

fn write_case(out: &mut String, fsm: &Fsm, type_name: &str, state: &str) -> Result<()> {
    let transitions = fsm
        .transitions
        .iter()
        .filter(|transition| transition.from == state)
        .collect::<Vec<_>>();
    if transitions.is_empty() {
        return Ok(());
    }
    writeln!(out, "\tcase {type_name}State{}:", go_type(state))?;
    for transition in transitions {
        write_if(out, transition, type_name)?;
    }
    Ok(())
}

fn write_if(out: &mut String, transition: &Transition, type_name: &str) -> Result<()> {
    let event = transition.on.as_deref().unwrap_or("");
    writeln!(out, "\t\tif event == \"{event}\" {{")?;
    writeln!(
        out,
        "\t\t\treturn {type_name}State{}, true",
        go_type(&transition.to)
    )?;
    writeln!(out, "\t\t}}")?;
    Ok(())
}
