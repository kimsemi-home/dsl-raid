use super::id;
use crate::commands::quality::agent_run::fields::{field_text, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    let manifest_hash = text(value, &["ssot", "core_ir_hash"]);
    let head_hash = field_text(item, "head_hash");
    if manifest_hash.is_some() && head_hash != manifest_hash {
        issues.push(format!(
            "semantic diff {} head_hash differs from ssot core hash",
            id(item)
        ));
    }
}
