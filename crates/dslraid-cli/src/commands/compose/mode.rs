use anyhow::{bail, Result};

pub(super) fn normalized_mode(materialize: &str) -> Result<String> {
    let mode = materialize.to_ascii_lowercase();
    if !matches!(
        mode.as_str(),
        "diagnostics-only" | "reachable" | "reachable-only" | "focus"
    ) {
        bail!("unsupported materialization mode: {materialize}");
    }
    Ok(mode)
}

pub(super) fn should_materialize(mode: &str) -> bool {
    mode != "diagnostics-only"
}

pub(super) fn focus_depth(mode: &str, depth: usize) -> usize {
    if mode == "focus" {
        depth.max(1)
    } else {
        usize::MAX
    }
}
