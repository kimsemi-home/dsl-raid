use super::model::TupleEdge;
use super::reachable_tail::finish_tuple_edge;
use super::tuple::{edge_subject, tuple_key, tuple_members, tuple_subject};
use dslraid_core::{Composition, Fsm, Transition};
use std::collections::{BTreeSet, VecDeque};

pub(crate) fn push_edges(
    composition: &Composition,
    fsms: &[&Fsm],
    tuple: &[String],
    queue: &mut VecDeque<Vec<String>>,
    seen: &BTreeSet<String>,
    edges: &mut Vec<TupleEdge>,
) {
    for (index, fsm) in fsms.iter().enumerate() {
        for transition in fsm
            .transitions
            .iter()
            .filter(|item| item.from == tuple[index])
        {
            let mut next = tuple.to_vec();
            next[index] = transition.to.clone();
            edges.push(tuple_edge(composition, fsms, tuple, &next, fsm, transition));
            if !seen.contains(&tuple_key(&next)) {
                queue.push_back(next);
            }
        }
    }
}

fn tuple_edge(
    composition: &Composition,
    fsms: &[&Fsm],
    from: &[String],
    to: &[String],
    fsm: &Fsm,
    transition: &Transition,
) -> TupleEdge {
    let edge = TupleEdge {
        subject: edge_subject(&composition.id, from, to, fsm, transition),
        from: tuple_subject(&composition.id, &tuple_members(fsms, from)),
        to: String::new(),
        members: Vec::new(),
        event: None,
    };
    finish_tuple_edge(composition, fsms, to, fsm, transition, edge)
}
