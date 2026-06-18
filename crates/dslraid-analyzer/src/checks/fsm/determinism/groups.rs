use std::collections::BTreeMap;

use dslraid_core::Fsm;

pub(super) type TransitionGroups = BTreeMap<(String, String), Vec<String>>;

pub(super) fn from_fsm(fsm: &Fsm) -> TransitionGroups {
    let mut by_key = BTreeMap::new();
    for transition in &fsm.transitions {
        by_key
            .entry((transition.from.clone(), event_key(transition.on.as_ref())))
            .or_insert_with(Vec::new)
            .push(transition.id.clone());
    }
    by_key
}

fn event_key(event: Option<&String>) -> String {
    event.cloned().unwrap_or_else(|| "epsilon".to_string())
}
