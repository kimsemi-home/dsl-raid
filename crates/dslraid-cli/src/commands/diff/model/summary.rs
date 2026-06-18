use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct DiffSummary {
    pub(crate) added: usize,
    pub(crate) removed: usize,
    pub(crate) changed: usize,
    pub(crate) by_kind: BTreeMap<String, DiffKindSummary>,
    pub(crate) review: DiffReviewSummary,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct DiffKindSummary {
    pub(crate) added: usize,
    pub(crate) removed: usize,
    pub(crate) changed: usize,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(crate) struct DiffReviewSummary {
    pub(crate) states_added: usize,
    pub(crate) states_removed: usize,
    pub(crate) states_changed: usize,
    pub(crate) transitions_added: usize,
    pub(crate) transitions_removed: usize,
    pub(crate) transitions_changed: usize,
    pub(crate) terminal_states_added: usize,
    pub(crate) terminal_states_removed: usize,
    pub(crate) terminal_paths_changed: usize,
    pub(crate) untested_transitions_added: usize,
    pub(crate) policy_traces_changed: usize,
}
