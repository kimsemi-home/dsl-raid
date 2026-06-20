import json

from actions_receipt_paths import has_private_path, read_text, workflow_files
from actions_receipt_policy import (
    generated_by,
    profile,
    required_checks,
    required_fields,
    required_kinds,
    required_rules,
    source,
    subject,
)
from actions_receipt_record import check_receipt


def check_manifest(data, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("actions receipt generator mismatch")
    if data.get("actions_receipt_profile") != profile:
        errors.append("actions receipt profile mismatch")
    if data.get("subject") != subject:
        errors.append("actions receipt subject mismatch")
    if data.get("source") != source:
        errors.append("actions receipt source mismatch")
    if has_private_path(text):
        errors.append("actions receipt leaked a private local path")


def by_kind(data, kind):
    return next((row for row in data.get("receipts", []) if row.get("kind") == kind), None)


def check_receipt_set(data, repo, errors):
    seen, kinds = set(), set()
    for row in data.get("receipts", []):
        check_receipt(row, repo, seen, kinds, errors)
    if required_kinds - kinds:
        errors.append(f"missing receipt kinds {sorted(required_kinds - kinds)}")
    workflow = by_kind(data, "workflow-summary")
    if workflow and required_fields - set(workflow.get("fields", [])):
        errors.append("workflow summary receipt misses remote verdict fields")
    pages = by_kind(data, "pages-health")
    if not pages or "HTTP/2 200" not in pages.get("expected", ""):
        errors.append("pages receipt must expect HTTP/2 200")


def check_workflows(repo, errors):
    if any("pull_request_target" in path.read_text(encoding="utf-8") for path in workflow_files(repo)):
        errors.append("workflow uses forbidden pull_request_target")
    branch = json.loads(read_text(repo, "docs/generated/verification-branch-protection.json"))
    checks = {r["name"] for r in branch.get("requirements", []) if r["kind"] == "required-check"}
    if required_checks - checks:
        errors.append("branch protection required checks are incomplete")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"actions receipt rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check actions receipts")


def collect_errors(data, repo, text):
    errors = []
    check_manifest(data, text, errors)
    check_receipt_set(data, repo, errors)
    check_workflows(repo, errors)
    check_rules(data, errors)
    return errors
