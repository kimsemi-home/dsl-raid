import os

from root_cause_case import check_case, generated_outputs
from root_cause_policy import (
    confidence_levels,
    generated_by,
    profile,
    required_cases,
    required_rules,
    source,
    statuses,
    subject,
)


def exists(repo, path):
    return os.path.exists(os.path.join(repo, path))


def check_manifest(data, repo, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("root cause generator mismatch")
    if data.get("root_cause_profile") != profile:
        errors.append("root cause profile mismatch")
    if data.get("subject") != subject:
        errors.append("root cause subject mismatch")
    if data.get("source") != source:
        errors.append("root cause source mismatch")
    if not exists(repo, source):
        errors.append(f"root cause missing source {source}")
    if "/" + "Users" + "/" in text:
        errors.append("root cause leaked a private local path")


def check_status(row, errors):
    cid = row.get("id", "")
    if row.get("status") not in statuses:
        errors.append(f"{cid} bad status")
    if row.get("confidence_ceiling") not in confidence_levels:
        errors.append(f"{cid} bad confidence ceiling")
    if row.get("status") != "confirmed" and row.get("confidence_ceiling") == "high":
        errors.append(f"{cid} unconfirmed root cause cannot be high confidence")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"root cause rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check root cause")


def collect_root_cause_errors(data, evidence, repo, text):
    errors, seen, cases = [], set(), []
    outputs = generated_outputs(evidence)
    check_manifest(data, repo, text, errors)
    for row in data.get("cases", []):
        check_status(row, errors)
        check_case(row, outputs, seen, cases, errors)
    if cases != required_cases:
        errors.append("root cause cases are not canonical")
    if not data.get("cases"):
        errors.append("root cause manifest has no cases")
    check_rules(data, errors)
    return errors
