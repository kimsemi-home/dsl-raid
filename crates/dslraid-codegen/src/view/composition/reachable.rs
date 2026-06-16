use super::expand::push_edges;
use super::model::{MaterializedComposition, TupleNode};
use super::reachable_tail::{all_flag, state_space_limit};
use super::tuple::{initial_tuple, tuple_key, tuple_members, tuple_subject};
use dslraid_core::{Composition, Fsm};
use std::collections::{BTreeSet, VecDeque};

pub(crate) fn materialize(composition: &Composition, fsms: &[&Fsm]) -> MaterializedComposition {
    let limit = state_space_limit(composition);
    let state_space = fsms.iter().map(|fsm| fsm.states.len().max(1)).product();
    let mut queue = VecDeque::from([initial_tuple(fsms)]);
    let mut seen = BTreeSet::new();
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut truncated = false;
    while let Some(tuple) = queue.pop_front() {
        if tuple.is_empty() || !seen.insert(tuple_key(&tuple)) {
            continue;
        }
        nodes.push(tuple_node(&composition.id, fsms, &tuple));
        if seen.len() >= limit {
            truncated = true;
            continue;
        }
        push_edges(composition, fsms, &tuple, &mut queue, &seen, &mut edges);
    }
    let visible: BTreeSet<_> = nodes.iter().map(|node| node.subject.as_str()).collect();
    edges.retain(|edge| visible.contains(edge.from.as_str()) && visible.contains(edge.to.as_str()));
    MaterializedComposition {
        state_space,
        truncated,
        nodes,
        edges,
    }
}

fn tuple_node(composition: &str, fsms: &[&Fsm], tuple: &[String]) -> TupleNode {
    let members = tuple_members(fsms, tuple);
    TupleNode {
        subject: tuple_subject(composition, &members),
        members,
        states: tuple.to_vec(),
        initial: all_flag(fsms, tuple, true),
        terminal: all_flag(fsms, tuple, false),
    }
}
