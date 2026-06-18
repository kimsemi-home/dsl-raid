mod catalog;

use catalog::{EvidenceCatalog, EvidenceStatus};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let refs = authority_refs(value);
    if refs.is_empty() {
        issues.push("approved authority gate requires evidence".to_string());
    }
    let evidence = evidence_kinds(value);
    let mut has_known_ref = false;
    let mut has_control_ref = false;
    for reference in &refs {
        match evidence.status(reference) {
            EvidenceStatus::Missing => {
                issues.push(format!(
                    "authority gate references unknown evidence {reference}"
                ));
            }
            EvidenceStatus::Pruned => {
                has_known_ref = true;
                issues.push(format!(
                    "authority gate references pruned evidence {reference}"
                ));
            }
            EvidenceStatus::ActiveControl => {
                has_known_ref = true;
                has_control_ref = true;
            }
            EvidenceStatus::ActiveOther => {
                has_known_ref = true;
            }
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

fn evidence_kinds(value: &Value) -> EvidenceCatalog {
    EvidenceCatalog::from_manifest(value)
}
