use super::fixtures::isolated_runscope_fixture;
use super::temp::temp_path;
use serde_json::Value;
use std::fs;

#[test]
fn generate_writes_validation_report() {
    let input = isolated_runscope_fixture();
    let out = temp_path("validation-report");
    super::super::run(crate::GenerateArgs {
        input: input.clone(),
        cli_doc: None,
        validation_report: Some(out.clone()),
        source_map: None,
        skip_lock: true,
    })
    .unwrap();

    let report: Value = serde_json::from_slice(&fs::read(&out).unwrap()).unwrap();
    fs::remove_file(&out).ok();
    fs::remove_file(&input).ok();
    assert_eq!(report["run"]["mode"].as_str(), Some("validate"));
    assert_eq!(report["summary"]["status"].as_str(), Some("passed"));
}
