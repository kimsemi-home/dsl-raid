use super::marks::DerivationMarks;
use serde_json::Value;
use std::collections::BTreeSet;

#[allow(clippy::too_many_arguments)]
pub(in crate::commands::query::index) fn push_query_item(
    items: &mut Vec<Value>,
    kind: &str,
    subject: &str,
    id: &str,
    label: &str,
    tags: &[String],
    visibility: Option<&str>,
    path: Option<&str>,
    marks: &DerivationMarks,
    extra: Option<Value>,
) {
    let tag_set: BTreeSet<&str> = tags.iter().map(String::as_str).collect();
    let mut item = serde_json::json!({
        "kind": kind,
        "subject": subject,
        "id": id,
        "label": label,
        "visibility": visibility,
        "tags": tags,
        "path": path,
        "tested": tag_set.contains("tested") || marks.tested.contains(subject),
        "generated": tag_set.contains("generated") || marks.generated.contains(subject)
    });
    if let Some(extra) = extra.and_then(|value| value.as_object().cloned()) {
        let object = item.as_object_mut().expect("query item is an object");
        for (key, value) in extra {
            object.insert(key, value);
        }
    }
    items.push(item);
}

pub(in crate::commands::query::index) fn push_basic_item(
    items: &mut Vec<Value>,
    kind: &str,
    id: &str,
    label: &str,
    tags: &[String],
    visibility: Option<&str>,
    marks: &DerivationMarks,
) {
    push_query_item(
        items, kind, id, id, label, tags, visibility, None, marks, None,
    );
}
