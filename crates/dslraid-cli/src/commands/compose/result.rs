use super::diagnostics::state_space_diagnostics;
use super::empty::empty_result;
use super::input::{resolve_input_fsms, state_space};
use super::mode::{focus_depth, normalized_mode, should_materialize};
use super::product::materialize_reachable_product;
use super::select::selected_composition;
use anyhow::{bail, Result};
use dslraid_core::CoreIr;
use serde_json::Value;

pub(crate) fn result(
    ir: &CoreIr,
    composition: Option<&str>,
    materialize: &str,
    limit: usize,
    focus: Option<&str>,
    depth: usize,
) -> Result<Value> {
    if limit == 0 {
        bail!("--limit must be greater than 0");
    }
    let Some(composition) = selected_composition(ir, composition) else {
        return Ok(empty_result(materialize, limit, focus, depth));
    };
    let input_fsms = resolve_input_fsms(ir, composition)?;
    let state_space = state_space(&input_fsms);
    let mode = normalized_mode(materialize)?;
    let diagnostics = state_space_diagnostics(&composition.id, state_space, limit);
    let (states, transitions, truncated) = if should_materialize(&mode) {
        materialize_reachable_product(
            &composition.id,
            &input_fsms,
            limit,
            focus,
            focus_depth(&mode, depth),
        )?
    } else {
        (Vec::new(), Vec::new(), false)
    };
    Ok(serde_json::json!({
        "composition_version": "0.1.0",
        "composition": {
            "id": composition.id,
            "name": composition.name,
            "kind": composition.kind,
            "inputs": composition.inputs,
            "mode": materialize,
            "state_space": state_space,
            "limit": limit,
            "lazy": true,
            "truncated": truncated,
            "focus": focus,
            "depth": depth
        },
        "states": states,
        "transitions": transitions,
        "diagnostics": diagnostics
    }))
}
