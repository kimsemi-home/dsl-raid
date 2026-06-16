use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiffReport {
    pub(crate) diff_version: &'static str,
    pub(crate) status: &'static str,
    pub(crate) base: DiffEndpoint,
    pub(crate) head: DiffEndpoint,
    pub(crate) summary: DiffSummary,
    pub(crate) changes: Vec<DiffChange>,
    pub(crate) warnings: Vec<DiffWarning>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiffEndpoint {
    pub(crate) path: String,
    pub(crate) hash: String,
    pub(crate) ir_version: String,
}

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

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiffChange {
    pub(crate) action: &'static str,
    pub(crate) kind: String,
    pub(crate) subject: String,
    pub(crate) label: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) before: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) after: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DiffWarning {
    pub(crate) code: &'static str,
    pub(crate) severity: &'static str,
    pub(crate) subject: String,
    pub(crate) message: String,
}
