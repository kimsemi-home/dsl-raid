mod consts;
mod transitions;

use anyhow::Result;
use dslraid_core::Fsm;
use std::fmt::Write;

use crate::names::go_type;

pub(super) fn write_fsm(out: &mut String, fsm: &Fsm) -> Result<()> {
    let type_name = go_type(&fsm.name);
    write_type(out, &type_name)?;
    consts::write_consts(out, fsm, &type_name)?;
    transitions::write(out, fsm, &type_name)?;
    Ok(())
}

fn write_type(out: &mut String, type_name: &str) -> Result<()> {
    writeln!(out, "type {type_name}State string\n")?;
    Ok(())
}
