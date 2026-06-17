use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn producer_cannot_be_listed_as_reviewer() {
    let mut value = base_manifest(
        json!([
            { "id": "agent:codex" },
            { "id": "reviewer:quality" }
        ]),
        "finished",
        high(),
    );
    value["orchestration"]["verified_by"] = json!("reviewer:quality");
    value["orchestration"]["selected_reviewers"] = json!(["reviewer:quality"]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["producer agent:codex cannot be listed as reviewer"]
    );
}
