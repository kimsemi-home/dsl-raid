use super::fixtures::{cleanup_isolated_fixture, isolated_runscope_fixture};
use super::temp::temp_path;
use serde_json::Value;
use std::fs;

#[test]
fn generate_writes_source_map() {
    let input = isolated_runscope_fixture();
    let out = temp_path("source-map");
    super::super::run(crate::GenerateArgs {
        input: input.clone(),
        cli_doc: None,
        validation_report: None,
        source_map: Some(out.clone()),
        skip_lock: true,
    })
    .unwrap();

    let map: Value = serde_json::from_slice(&fs::read(&out).unwrap()).unwrap();
    fs::remove_file(&out).ok();
    cleanup_isolated_fixture(&input);
    assert_eq!(map["source_map_version"].as_str(), Some("0.1.0"));
    assert!(contains_subject(
        &map,
        "transition:runtime.running_to_completed"
    ));
}

fn contains_subject(map: &Value, subject: &str) -> bool {
    map["mappings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["ir_subject"].as_str() == Some(subject))
}
