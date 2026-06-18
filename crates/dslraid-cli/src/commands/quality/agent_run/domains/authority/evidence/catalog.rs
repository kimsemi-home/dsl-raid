use super::super::super::fields::{field_is, field_text, items};
use serde_json::Value;
use std::collections::BTreeMap;

pub(super) enum EvidenceStatus {
    Missing,
    Pruned,
    ActiveControl,
    ActiveOther,
}

pub(super) struct EvidenceCatalog {
    entries: BTreeMap<String, EvidenceMeta>,
}

struct EvidenceMeta {
    kind: String,
    pruned: bool,
}

impl EvidenceCatalog {
    pub(super) fn from_manifest(value: &Value) -> Self {
        Self {
            entries: items(value, "evidence").filter_map(meta_entry).collect(),
        }
    }

    pub(super) fn status(&self, id: &str) -> EvidenceStatus {
        let Some(meta) = self.entries.get(id) else {
            return EvidenceStatus::Missing;
        };
        if meta.pruned {
            return EvidenceStatus::Pruned;
        }
        if is_control_kind(&meta.kind) {
            return EvidenceStatus::ActiveControl;
        }
        EvidenceStatus::ActiveOther
    }
}

fn meta_entry(item: &Value) -> Option<(String, EvidenceMeta)> {
    let id = field_text(item, "id")?.to_string();
    let kind = field_text(item, "kind")?.to_string();
    let pruned = field_is(item, "status", "pruned");
    Some((id, EvidenceMeta { kind, pruned }))
}

fn is_control_kind(kind: &str) -> bool {
    matches!(kind, "validation" | "decision")
}
