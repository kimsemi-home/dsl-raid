use anyhow::Result;
use std::fmt::Write;

pub(super) fn write(out: &mut String, type_name: &str, table: &str) -> Result<()> {
    writeln!(
        out,
        "func {type_name}Transition(state {type_name}State, event string) ({type_name}State, bool) {{"
    )?;
    writeln!(out, "\treturn dslraidNext(state, event, {table})")?;
    writeln!(out, "}}")?;
    Ok(())
}
