use anyhow::{bail, Result};
use dslraid_codegen::{generate_code, CodegenTarget};
use dslraid_core::CoreIr;
use serde_json::Value;
use std::path::Path;

pub(super) fn check(ir: &CoreIr, input: &Path) -> Result<()> {
    check_codegen(ir)?;
    check_queries(ir)?;
    check_composition(ir)?;
    check_self_diff(input, ir)
}

fn check_codegen(ir: &CoreIr) -> Result<()> {
    for target in [
        CodegenTarget::Rust,
        CodegenTarget::Go,
        CodegenTarget::TypeScript,
    ] {
        let generated = generate_code(ir, target)?;
        if generated.trim().is_empty() {
            bail!("empty codegen output for {target:?}");
        }
    }
    Ok(())
}

fn check_queries(ir: &CoreIr) -> Result<()> {
    let transition_query = crate::commands::query::values(ir, "kind=transition")?;
    if transition_query.is_empty() {
        bail!("query returned no transitions");
    }
    let richer_query = crate::commands::query::values(
        ir,
        "kind=transition and requires~=policy:no_secret_leak or terminal=true",
    )?;
    if richer_query.is_empty() {
        bail!("richer query returned no results");
    }
    Ok(())
}

fn check_composition(ir: &CoreIr) -> Result<()> {
    let composition = crate::commands::compose::result(ir, None, "reachable", 100, None, 1)?;
    let state_space = composition
        .get("composition")
        .and_then(|value| value.get("state_space"))
        .and_then(Value::as_u64)
        .unwrap_or_default();
    if state_space == 0 {
        bail!("lazy composition did not compute a state space");
    }
    Ok(())
}

fn check_self_diff(input: &Path, ir: &CoreIr) -> Result<()> {
    let self_diff = crate::commands::diff::report(ir, ir, input, input)?;
    if self_diff.status != "unchanged" {
        bail!("self diff should be unchanged");
    }
    Ok(())
}
