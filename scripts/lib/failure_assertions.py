import os

from failure_condition import check_condition, generated_outputs
from failure_policy import (
    domains,
    generated_by,
    profile,
    required_conditions,
    required_rules,
    severities,
    source,
    subject,
)


def exists(repo, path):
    return os.path.exists(os.path.join(repo, path))


def check_manifest(data, repo, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("failure generator mismatch")
    if data.get("failure_profile") != profile:
        errors.append("failure profile mismatch")
    if data.get("subject") != subject:
        errors.append("failure subject mismatch")
    if data.get("source") != source:
        errors.append("failure source mismatch")
    if not exists(repo, source):
        errors.append(f"failure missing source {source}")
    if "/" + "Users" + "/" in text:
        errors.append("failure leaked a private local path")


def check_domain_and_severity(row, errors):
    cid = row.get("id", "")
    if row.get("domain") not in domains:
        errors.append(f"{cid} bad domain")
    if row.get("severity") not in severities:
        errors.append(f"{cid} bad severity")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"failure rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check failure conditions")


def collect_failure_errors(data, evidence, repo, text):
    errors, seen, ordered = [], set(), []
    outputs = generated_outputs(evidence)
    check_manifest(data, repo, text, errors)
    for row in data.get("conditions", []):
        check_domain_and_severity(row, errors)
        check_condition(row, outputs, seen, ordered, errors)
    if ordered != required_conditions:
        errors.append("failure conditions are not canonical")
    if not data.get("conditions"):
        errors.append("failure manifest has no conditions")
    check_rules(data, errors)
    return errors
