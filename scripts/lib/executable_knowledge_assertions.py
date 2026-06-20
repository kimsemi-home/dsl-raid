from executable_knowledge_policy import (
    generated_by,
    profile,
    required_kinds,
    required_rules,
    subject,
)
from executable_knowledge_record import check_record, exists


def check_manifest(data, repo, errors):
    if data.get("generated_by") != generated_by:
        errors.append("executable knowledge generator mismatch")
    if data.get("executable_knowledge_profile") != profile:
        errors.append("executable knowledge profile mismatch")
    if data.get("subject") != subject:
        errors.append("executable knowledge subject mismatch")
    source = data.get("source", "")
    if not exists(repo, source):
        errors.append(f"executable knowledge missing source {source}")


def check_closure_rules(data, errors):
    found = {row["id"] for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"executable knowledge rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row['id']} must self-check executable knowledge")

def collect_executable_knowledge_errors(data, repo):
    errors, seen, kinds = [], set(), []
    check_manifest(data, repo, errors)
    for row in data.get("records", []):
        check_record(row, repo, seen, kinds, errors)
    if kinds != required_kinds:
        errors.append("executable knowledge kinds are not canonical")
    check_closure_rules(data, errors)
    return errors
