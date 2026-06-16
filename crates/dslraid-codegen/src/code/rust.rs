use anyhow::Result;
use dslraid_core::{fsm_local_name, Fsm, Transition};
use std::fmt::Write;

use crate::names::{rust_type, snake};

pub(super) fn write_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    let enum_name = rust_type(&fsm.name);
    write_enum(out, fsm, &enum_name)?;
    write_transition(out, fsm, &enum_name)?;
    Ok(())
}

fn write_enum(out: &mut String, fsm: &Fsm, enum_name: &str) -> Result<()> {
    writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]")?;
    writeln!(out, "pub enum {enum_name}State {{")?;
    for state in &fsm.states {
        writeln!(out, "    {},", rust_type(&state.id))?;
    }
    writeln!(out, "}}\n")?;
    Ok(())
}

fn write_transition(out: &mut String, fsm: &Fsm, enum_name: &str) -> Result<()> {
    writeln!(
        out,
        "pub fn {}_transition(state: {enum_name}State, event: Option<&str>) -> Option<{enum_name}State> {{",
        snake(fsm_local_name(&fsm.id))
    )?;
    writeln!(out, "    match (state, event) {{")?;
    for transition in &fsm.transitions {
        write_arm(out, transition, enum_name)?;
    }
    writeln!(out, "        _ => None,")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}\n")?;
    Ok(())
}

fn write_arm(out: &mut String, transition: &Transition, enum_name: &str) -> Result<()> {
    let event = transition
        .on
        .as_ref()
        .map(|event| format!("Some(\"{}\")", event))
        .unwrap_or_else(|| "None".to_string());
    writeln!(
        out,
        "        ({enum_name}State::{}, {}) => Some({enum_name}State::{}),",
        rust_type(&transition.from),
        event,
        rust_type(&transition.to)
    )?;
    Ok(())
}
