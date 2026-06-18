use crate::commands::quality::agent_run::fields::{field_text, items};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) struct RefCatalog {
    reviewers: BTreeSet<String>,
    evidence: BTreeSet<String>,
    artifacts: BTreeSet<String>,
}

impl RefCatalog {
    pub(super) fn from_manifest(value: &Value) -> Self {
        Self {
            reviewers: ids(value, "reviewers"),
            evidence: ids(value, "evidence"),
            artifacts: ids(value, "artifacts"),
        }
    }

    pub(super) fn reviewers(&self) -> &BTreeSet<String> {
        &self.reviewers
    }

    pub(super) fn evidence(&self) -> &BTreeSet<String> {
        &self.evidence
    }

    pub(super) fn artifacts(&self) -> &BTreeSet<String> {
        &self.artifacts
    }
}

fn ids(value: &Value, key: &str) -> BTreeSet<String> {
    items(value, key)
        .filter_map(|item| field_text(item, "id").map(str::to_string))
        .collect()
}
