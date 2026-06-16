use super::expand::{push_next_edges, Expansion};
use super::focus::tuple_matches_focus;
use super::tuple::{tuple_key, tuple_state_value};
use super::value::value_string;
use anyhow::{anyhow, Result};
use dslraid_core::Fsm;
use serde_json::Value;
use std::collections::{BTreeSet, VecDeque};

pub(super) fn materialize_reachable_product(
    composition_id: &str,
    fsms: &[&Fsm],
    limit: usize,
    focus: Option<&str>,
    focus_depth: usize,
) -> Result<(Vec<Value>, Vec<Value>, bool)> {
    if fsms.is_empty() {
        return Ok((Vec::new(), Vec::new(), false));
    }
    let initial = initial_tuple(fsms)?;
    let mut queue = VecDeque::from([(initial, 0usize)]);
    let mut seen = BTreeSet::new();
    let mut states = Vec::new();
    let mut transitions = Vec::new();
    let mut truncated = false;

    while let Some((tuple, depth)) = queue.pop_front() {
        let current_key = tuple_key(&tuple);
        if !seen.insert(current_key) {
            continue;
        }
        if seen.len() > limit {
            truncated = true;
            break;
        }
        if focus
            .is_none_or(|subject| tuple_matches_focus(fsms, &tuple, subject, focus_depth, depth))
        {
            states.push(tuple_state_value(composition_id, fsms, &tuple)?);
        }
        if focus.is_some() && depth >= focus_depth {
            continue;
        }
        let mut expansion = Expansion {
            composition_id,
            fsms,
            focus,
            seen: &mut seen,
            queue: &mut queue,
            transitions: &mut transitions,
            truncated: &mut truncated,
            limit,
            depth,
        };
        push_next_edges(&mut expansion, &tuple)?;
    }
    states.sort_by_key(|left| value_string(left, "id"));
    transitions.sort_by_key(|left| value_string(left, "id"));
    Ok((states, transitions, truncated))
}

fn initial_tuple(fsms: &[&Fsm]) -> Result<Vec<String>> {
    fsms.iter()
        .map(|fsm| {
            fsm.states
                .iter()
                .find(|state| state.initial)
                .or_else(|| fsm.states.first())
                .map(|state| state.id.clone())
                .ok_or_else(|| anyhow!("{} has no states", fsm.id))
        })
        .collect()
}
