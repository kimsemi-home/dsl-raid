use anyhow::Result;
use dslraid_core::{fsm_local_name, Fsm, Transition};
use std::fmt::Write;

use crate::names::{camel, rust_type};

pub(super) fn write_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    let const_name = rust_type(&fsm.name);
    write_type(out, fsm, &const_name)?;
    write_transition(out, fsm, &const_name)?;
    Ok(())
}

fn write_type(out: &mut String, fsm: &Fsm, const_name: &str) -> Result<()> {
    writeln!(
        out,
        "export type {const_name}State = {};",
        fsm.states
            .iter()
            .map(|state| format!("\"{}\"", state.id))
            .collect::<Vec<_>>()
            .join(" | ")
    )?;
    Ok(())
}

fn write_transition(out: &mut String, fsm: &Fsm, const_name: &str) -> Result<()> {
    writeln!(
        out,
        "export function {}Transition(state: {const_name}State, event?: string): {const_name}State | undefined {{",
        camel(fsm_local_name(&fsm.id))
    )?;
    writeln!(out, "  switch (`${{state}}:${{event ?? \"\"}}`) {{")?;
    for transition in &fsm.transitions {
        write_case(out, transition)?;
    }
    writeln!(out, "    default: return undefined;")?;
    writeln!(out, "  }}")?;
    writeln!(out, "}}\n")?;
    Ok(())
}

fn write_case(out: &mut String, transition: &Transition) -> Result<()> {
    writeln!(
        out,
        "    case \"{}:{}\": return \"{}\";",
        transition.from,
        transition.on.as_deref().unwrap_or(""),
        transition.to
    )?;
    Ok(())
}
