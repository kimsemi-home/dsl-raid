use serde_json::{json, Value};

pub(super) fn with_links(items: &mut [Value]) {
    let ids = ids(items);
    if ids.len() < 2 {
        return;
    }
    for (index, item) in items.iter_mut().enumerate() {
        if item.get("links").is_none() {
            let target = if index == 0 { &ids[1] } else { &ids[0] };
            item["links"] = json!([{ "relation": "corroborates", "target": target }]);
        }
    }
}

fn ids(items: &[Value]) -> Vec<String> {
    items
        .iter()
        .filter_map(|item| item.get("id").and_then(Value::as_str))
        .map(str::to_string)
        .collect()
}
