import os

from failure_recovery_context import build_context
from failure_recovery_policy import (
    generated_by,
    profile,
    required_recoveries,
    required_rules,
    source,
    subject,
)
from failure_recovery_record import check_recovery


def exists(repo, path):
    return os.path.exists(os.path.join(repo, path))


def check_manifest(data, repo, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("failure recovery generator mismatch")
    if data.get("failure_recovery_profile") != profile:
        errors.append("failure recovery profile mismatch")
    if data.get("subject") != subject:
        errors.append("failure recovery subject mismatch")
    if data.get("source") != source:
        errors.append("failure recovery source mismatch")
    if not exists(repo, source):
        errors.append(f"failure recovery missing source {source}")
    if "/" + "Users" + "/" in text:
        errors.append("failure recovery leaked a private local path")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"failure recovery rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check recovery")


def collect_recovery_errors(data, sources, repo, text):
    errors, seen, ordered = [], set(), []
    context = build_context(*sources)
    check_manifest(data, repo, text, errors)
    for row in data.get("recoveries", []):
        check_recovery(row, context, seen, ordered, errors)
    if ordered != required_recoveries:
        errors.append("failure recoveries are not canonical")
    if not data.get("recoveries"):
        errors.append("failure recovery manifest has no records")
    check_rules(data, errors)
    return errors
