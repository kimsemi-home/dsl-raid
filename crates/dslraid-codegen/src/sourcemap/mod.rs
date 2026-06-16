mod entry;
mod generated;
mod go;
mod go_items;
mod lines;
mod location;
mod push;
mod rust;
mod rust_items;

use dslraid_core::CoreIr;
use serde_json::{json, Value};

pub fn generate_source_map(ir: &CoreIr, design_path: &str) -> Value {
    let generated = generated::build(ir);
    let design_ir = design_ir(ir, design_path);
    json!({
        "source_map_version": "0.1.0",
        "design_ir": design_ir,
        "mappings": entry::mappings(ir, &generated)
    })
}

fn design_ir(ir: &CoreIr, design_path: &str) -> Value {
    let mut value = json!({ "path": design_path });
    if let Ok(hash) = dslraid_core::sha256_json(ir) {
        value["hash"] = Value::String(hash);
    }
    value
}
