use std::collections::BTreeSet;

use dslraid_core::Transition;
use serde_json::{json, Map, Value};

pub(super) fn state(
    states: &BTreeSet<String>,
    failures: &mut Vec<Value>,
    transition: &str,
    key: &str,
    state: &str,
) {
    if !states.contains(state) {
        failures.push(ref_failure(transition, key, state));
    }
}

pub(super) fn event(
    events: &BTreeSet<String>,
    failures: &mut Vec<Value>,
    transition: &str,
    source: &Transition,
) {
    if let Some(event) = &source.on {
        if !events.contains(event) {
            failures.push(json!({ "transition": transition, "event": event }));
        }
    }
}

pub(super) fn refs(
    known: &BTreeSet<String>,
    failures: &mut Vec<Value>,
    transition: &str,
    key: &str,
    refs: &[String],
) {
    for reference in refs {
        if !known.contains(reference) {
            failures.push(ref_failure(transition, key, reference));
        }
    }
}

fn ref_failure(transition: &str, key: &str, reference: &str) -> Value {
    let mut item = Map::new();
    item.insert("transition".to_string(), json!(transition));
    item.insert(key.to_string(), json!(reference));
    Value::Object(item)
}
