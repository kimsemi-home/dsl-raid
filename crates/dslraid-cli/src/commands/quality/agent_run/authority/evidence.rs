use super::super::fields::{field_is, field_text, items};
use serde_json::Value;
use std::collections::BTreeMap;

struct EvidenceMeta {
    kind: String,
    pruned: bool,
}

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let refs = authority_refs(value);
    if refs.is_empty() {
        issues.push("approved authority gate requires evidence".to_string());
    }
    let evidence = evidence_kinds(value);
    let mut has_known_ref = false;
    let mut has_control_ref = false;
    for reference in &refs {
        if let Some(meta) = evidence.get(*reference) {
            has_known_ref = true;
            if meta.pruned {
                issues.push(format!(
                    "authority gate references pruned evidence {reference}"
                ));
                continue;
            }
            has_control_ref |= is_control_kind(&meta.kind);
        } else {
            issues.push(format!(
                "authority gate references unknown evidence {reference}"
            ));
        }
    }
    if has_known_ref && !has_control_ref {
        issues.push("approved authority gate requires validation or decision evidence".to_string());
    }
}

fn authority_refs(value: &Value) -> Vec<&str> {
    value
        .get("authority_gate")
        .and_then(|gate| gate.get("evidence"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}

fn evidence_kinds(value: &Value) -> BTreeMap<String, EvidenceMeta> {
    items(value, "evidence")
        .filter_map(|item| {
            let id = field_text(item, "id")?;
            let kind = field_text(item, "kind")?;
            Some((
                id.to_string(),
                EvidenceMeta {
                    kind: kind.to_string(),
                    pruned: field_is(item, "status", "pruned"),
                },
            ))
        })
        .collect()
}

fn is_control_kind(kind: &str) -> bool {
    matches!(kind, "validation" | "decision")
}
