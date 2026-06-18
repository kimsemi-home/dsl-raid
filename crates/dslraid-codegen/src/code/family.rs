use anyhow::Result;
use dslraid_core::{CoreIr, Fsm};

pub(super) fn write_family(
    out: &mut String,
    ir: &CoreIr,
    writer: fn(&mut String, &Fsm) -> Result<()>,
    header: &str,
) -> Result<()> {
    out.push_str(header);
    for (index, fsm) in ir.fsms.iter().enumerate() {
        writer(out, fsm)?;
        if needs_family_gap(out, index, ir.fsms.len()) {
            out.push('\n');
        }
    }
    Ok(())
}

fn needs_family_gap(out: &str, index: usize, len: usize) -> bool {
    index + 1 < len && !out.ends_with("\n\n")
}
