mod expand;
mod focus;
mod id;
mod initial;
mod materialize;
mod tuple;
mod value;

use anyhow::Result;
use dslraid_core::Fsm;
use serde_json::Value;

pub(super) fn materialize_reachable_product(
    composition_id: &str,
    fsms: &[&Fsm],
    limit: usize,
    focus: Option<&str>,
    focus_depth: usize,
) -> Result<(Vec<Value>, Vec<Value>, bool)> {
    materialize::materialize_reachable_product(composition_id, fsms, limit, focus, focus_depth)
}
