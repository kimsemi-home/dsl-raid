use super::focus::transition_matches_focus;
use super::tuple::{tuple_key, tuple_transition_value};
use anyhow::Result;
use dslraid_core::Fsm;
use serde_json::Value;
use std::collections::{BTreeSet, VecDeque};

pub(super) struct Expansion<'a, 'b> {
    pub(super) composition_id: &'a str,
    pub(super) fsms: &'a [&'a Fsm],
    pub(super) focus: Option<&'a str>,
    pub(super) seen: &'b mut BTreeSet<String>,
    pub(super) queue: &'b mut VecDeque<(Vec<String>, usize)>,
    pub(super) transitions: &'b mut Vec<Value>,
    pub(super) truncated: &'b mut bool,
    pub(super) limit: usize,
    pub(super) depth: usize,
}

pub(super) fn push_next_edges(expansion: &mut Expansion<'_, '_>, tuple: &[String]) -> Result<()> {
    for (index, fsm) in expansion.fsms.iter().enumerate() {
        let current = &tuple[index];
        for transition in fsm
            .transitions
            .iter()
            .filter(|transition| &transition.from == current)
        {
            let mut next_tuple = tuple.to_vec();
            next_tuple[index] = transition.to.clone();
            let next_key = tuple_key(&next_tuple);
            if expansion.seen.len() + expansion.queue.len() >= expansion.limit
                && !expansion.seen.contains(&next_key)
            {
                *expansion.truncated = true;
                continue;
            }
            let edge = tuple_transition_value(
                expansion.composition_id,
                expansion.fsms,
                tuple,
                &next_tuple,
                &fsm.id,
                transition,
            )?;
            if expansion
                .focus
                .is_none_or(|subject| transition_matches_focus(&edge, subject))
            {
                expansion.transitions.push(edge);
            }
            if !expansion.seen.contains(&next_key) {
                expansion.queue.push_back((next_tuple, expansion.depth + 1));
            }
        }
    }
    Ok(())
}
