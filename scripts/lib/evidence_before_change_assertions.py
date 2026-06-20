import os

from evidence_before_change_change import check_change, generated_outputs
from evidence_before_change_policy import (
    generated_by,
    profile,
    required_changes,
    required_rules,
    source,
    subject,
)


def exists(repo, path):
    return os.path.exists(os.path.join(repo, path))


def check_manifest(data, repo, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("evidence-before-change generator mismatch")
    if data.get("evidence_before_change_profile") != profile:
        errors.append("evidence-before-change profile mismatch")
    if data.get("subject") != subject:
        errors.append("evidence-before-change subject mismatch")
    if data.get("source") != source:
        errors.append("evidence-before-change source mismatch")
    if not exists(repo, source):
        errors.append(f"evidence-before-change missing source {source}")
    if "/" + "Users" + "/" in text:
        errors.append("evidence-before-change leaked a private local path")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"evidence-before-change rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check evidence gate")


def collect_evidence_before_change_errors(data, evidence, repo, text):
    errors, seen, changes = [], set(), []
    outputs = generated_outputs(evidence)
    check_manifest(data, repo, text, errors)
    for row in data.get("changes", []):
        check_change(row, outputs, seen, changes, errors)
    if changes != required_changes:
        errors.append("evidence-before-change changes are not canonical")
    if not data.get("changes"):
        errors.append("evidence-before-change has no changes")
    check_rules(data, errors)
    return errors
