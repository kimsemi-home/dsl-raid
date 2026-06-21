from verification_feedback_closure import check_closure
from verification_feedback_evidence import generated_outputs
from verification_feedback_paths import has_private_path
from verification_feedback_policy import (
    generated_by,
    profile,
    required_closures,
    required_rules,
    source,
    subject,
)


def check_manifest(data, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("feedback generator mismatch")
    if data.get("feedback_profile") != profile:
        errors.append("feedback profile mismatch")
    if data.get("subject") != subject:
        errors.append("feedback subject mismatch")
    if data.get("source") != source:
        errors.append("feedback source mismatch")
    if has_private_path(text):
        errors.append("feedback manifest leaked a private local path")


def check_closures(data, outputs, errors):
    seen, closures = set(), set()
    for row in data.get("closures", []):
        check_closure(row, outputs, seen, closures, errors)
    if not data.get("closures"):
        errors.append("feedback manifest has no closures")
    missing = required_closures - closures
    if missing:
        errors.append(f"missing feedback closures {sorted(missing)}")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if not found:
        errors.append("feedback manifest has no closure rules")
    if found != required_rules:
        errors.append(f"feedback rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check feedback")


def collect_errors(data, evidence, text):
    errors = []
    check_manifest(data, text, errors)
    check_closures(data, generated_outputs(evidence), errors)
    check_rules(data, errors)
    return errors
