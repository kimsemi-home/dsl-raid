use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn generate_writes_validation_report() {
    let out = temp_path("validation-report");
    super::run(crate::GenerateArgs {
        input: repo_path("examples/runscope/runscope.raid.json"),
        cli_doc: None,
        validation_report: Some(out.clone()),
        source_map: None,
        skip_lock: true,
    })
    .unwrap();

    let report: Value = serde_json::from_slice(&fs::read(&out).unwrap()).unwrap();
    fs::remove_file(&out).ok();
    assert_eq!(report["run"]["mode"].as_str(), Some("validate"));
    assert_eq!(report["summary"]["status"].as_str(), Some("passed"));
}

#[test]
fn generate_writes_source_map() {
    let out = temp_path("source-map");
    super::run(crate::GenerateArgs {
        input: repo_path("examples/runscope/runscope.raid.json"),
        cli_doc: None,
        validation_report: None,
        source_map: Some(out.clone()),
        skip_lock: true,
    })
    .unwrap();

    let map: Value = serde_json::from_slice(&fs::read(&out).unwrap()).unwrap();
    fs::remove_file(&out).ok();
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

fn repo_path(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "dslraid-generate-{name}-{}-{}.json",
        std::process::id(),
        timestamp()
    ))
}

fn timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos()
}
