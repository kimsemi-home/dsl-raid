import os

from release_provenance_policy import expected_gates, release_workflow, workflow_tokens


def read_workflow(errors):
    if not os.path.exists(release_workflow):
        errors.append(f"missing path {release_workflow}")
        return ""
    return open(release_workflow, encoding="utf-8").read()


def check_workflow_text(text, errors):
    for token in workflow_tokens:
        if token not in text:
            errors.append(f"release workflow missing {token}")
    for forbidden in ("pull_request_target:", "write-all"):
        if forbidden in text:
            errors.append(f"release workflow uses forbidden {forbidden}")


def check_actions_manifest(actions, errors):
    rows = [
        row for row in actions.get("workflows", [])
        if row["workflow"] == release_workflow
    ]
    if len(rows) != 1:
        errors.append("release workflow must appear once in GitHub Actions suite")
        return
    if rows[0]["generator"] != "scripts/releasegen.sh":
        errors.append("release workflow generator mismatch")
    if rows[0]["role"] != "generated-release":
        errors.append("release workflow role mismatch")


def check_gate(row, text, seen, errors):
    rid = row["id"]
    if rid in seen:
        errors.append(f"duplicate release gate {rid}")
    seen.add(rid)
    if rid not in expected_gates:
        errors.append(f"unexpected release gate {rid}")
    if row.get("status") != "required":
        errors.append(f"{rid} must be required")
    if row["workflow"] != release_workflow:
        errors.append(f"{rid} must use release workflow")
    for item in row.get("requires", []):
        if item not in text:
            errors.append(f"{rid} missing release requirement {item}")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{rid} missing evidence {item}")
    if not row.get("evidence"):
        errors.append(f"{rid} must cite evidence")


def check_gates(manifest, text, errors):
    seen = set()
    for gate in manifest.get("gates", []):
        check_gate(gate, text, seen, errors)
    if seen != expected_gates:
        errors.append(f"release gate set mismatch: {sorted(seen)}")


def collect_release_provenance_errors(manifest, actions):
    errors = []
    text = read_workflow(errors)
    check_workflow_text(text, errors)
    check_actions_manifest(actions, errors)
    check_gates(manifest, text, errors)
    if not manifest.get("closure_rules"):
        errors.append("release provenance manifest has no rules")
    return errors
