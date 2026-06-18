use std::collections::{BTreeSet, VecDeque};

use dslraid_core::Fsm;

pub(super) fn states(fsm: &Fsm) -> BTreeSet<String> {
    let mut reachable = BTreeSet::new();
    let Some(initial) = fsm.states.iter().find(|state| state.initial) else {
        return reachable;
    };
    let mut queue = VecDeque::from([initial.id.clone()]);
    while let Some(state) = queue.pop_front() {
        push_next(fsm, &mut reachable, &mut queue, state);
    }
    reachable
}

fn push_next(
    fsm: &Fsm,
    reachable: &mut BTreeSet<String>,
    queue: &mut VecDeque<String>,
    state: String,
) {
    if reachable.insert(state.clone()) {
        queue.extend(next_states(fsm, &state));
    }
}

fn next_states(fsm: &Fsm, state: &str) -> Vec<String> {
    fsm.transitions
        .iter()
        .filter(move |transition| transition.from == state)
        .map(|transition| transition.to.clone())
        .collect()
}
