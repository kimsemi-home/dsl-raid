use super::{DiffChange, DiffEndpoint, DiffSummary, DiffWarning};
use serde::Serialize;

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
