use std::collections::BTreeMap;

use dslraid_core::{Artifact, CoreIr};
use serde_json::Value;

pub(super) type Index = BTreeMap<String, Vec<Value>>;

pub(super) fn build(ir: &CoreIr) -> Index {
    let mut index = Index::new();
    for artifact in ir
        .artifacts
        .iter()
        .filter(|artifact| artifact.kind == "generated")
    {
        add_artifact(ir, artifact, &mut index);
    }
    index
}

pub(super) fn push(index: &mut Index, subject: String, location: Value) {
    index.entry(subject).or_default().push(location);
}

fn add_artifact(ir: &CoreIr, artifact: &Artifact, index: &mut Index) {
    match extension(&artifact.path) {
        Some("rs") => super::rust::add(ir, artifact, index),
        Some("go") => super::go::add(ir, artifact, index),
        _ => {}
    }
}

fn extension(path: &str) -> Option<&str> {
    std::path::Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
}
