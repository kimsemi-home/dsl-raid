use anyhow::Result;

pub(super) fn check() -> Result<()> {
    for script in super::super::scripts::generated_doc_scripts() {
        super::script_check::check(script)?;
    }
    Ok(())
}
