use serde_json::Value;

pub(super) fn print_items(items: Vec<Value>) {
    for item in items {
        print_item(&item);
    }
}

fn print_item(item: &Value) {
    println!(
        "{} {} {}",
        field(item, "kind", "unknown"),
        field(item, "subject", "<unknown>"),
        field(item, "label", "")
    );
}

fn field<'a>(item: &'a Value, name: &str, fallback: &'a str) -> &'a str {
    item.get(name).and_then(Value::as_str).unwrap_or(fallback)
}
