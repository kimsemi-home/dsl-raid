use super::model::ToolRecord;

pub(super) fn dslraid() -> ToolRecord {
    named("dslraid", "0.1.0")
}

pub(super) fn named(name: &str, version: &str) -> ToolRecord {
    ToolRecord {
        name: name.to_string(),
        version: version.to_string(),
    }
}
