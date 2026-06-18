mod function;
mod table;

use anyhow::Result;
use dslraid_core::Fsm;

pub(super) fn write(out: &mut String, fsm: &Fsm, type_name: &str) -> Result<()> {
    let table = table::name(fsm);
    table::write(out, fsm, type_name, &table)?;
    function::write(out, type_name, &table)
}
