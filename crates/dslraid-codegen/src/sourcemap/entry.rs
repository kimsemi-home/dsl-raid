use dslraid_core::{state_subject, transition_subject, CoreIr, Fsm};
use serde_json::{Map, Value};

use super::generated::Index;

pub(super) fn mappings(ir: &CoreIr, generated: &Index) -> Vec<Value> {
    let mut values = Vec::new();
    for fsm in &ir.fsms {
        values.push(mapping(&fsm.id, &fsm.defined_at, generated));
        values.extend(state_mappings(fsm, generated));
        values.extend(transition_mappings(fsm, generated));
    }
    values
}

fn state_mappings(fsm: &Fsm, generated: &Index) -> Vec<Value> {
    fsm.states
        .iter()
        .map(|state| {
            mapping(
                &state_subject(&fsm.id, &state.id),
                &state.defined_at,
                generated,
            )
        })
        .collect()
}

fn transition_mappings(fsm: &Fsm, generated: &Index) -> Vec<Value> {
    fsm.transitions
        .iter()
        .map(|transition| {
            mapping(
                &transition_subject(&fsm.id, &transition.id),
                &transition.defined_at,
                generated,
            )
        })
        .collect()
}

fn mapping(
    subject: &str,
    defined_at: &Option<dslraid_core::DefinedAt>,
    generated: &Index,
) -> Value {
    let mut object = Map::new();
    object.insert("id".to_string(), Value::String(mapping_id(subject)));
    object.insert("ir_subject".to_string(), Value::String(subject.to_string()));
    if let Some(location) = super::location::dsl(defined_at) {
        object.insert("dsl_location".to_string(), location);
    }
    object.insert(
        "generated_locations".to_string(),
        Value::Array(generated.get(subject).cloned().unwrap_or_default()),
    );
    Value::Object(object)
}

fn mapping_id(subject: &str) -> String {
    format!("map-{}", subject.replace([':', '.'], "-"))
}
