from semantic_debugger_evidence import generated_outputs
from semantic_debugger_policy import (
    generated_by,
    profile,
    required_rules,
    source,
    subject,
)
from semantic_debugger_session import check_session


def check_manifest(data, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("semantic debugger generator mismatch")
    if data.get("semantic_debugger_profile") != profile:
        errors.append("semantic debugger profile mismatch")
    if data.get("subject") != subject:
        errors.append("semantic debugger subject mismatch")
    if data.get("source") != source:
        errors.append("semantic debugger source mismatch")
    if "/" + "Users" + "/" in text:
        errors.append("semantic debugger leaked a private local path")


def check_sessions(data, evidence, errors):
    outputs = generated_outputs(evidence)
    seen = set()
    for row in data.get("sessions", []):
        check_session(row, outputs, seen, errors)
    if not data.get("sessions"):
        errors.append("semantic debugger manifest has no sessions")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"semantic debugger rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check debugger")


def collect_errors(data, evidence, text):
    errors = []
    check_manifest(data, text, errors)
    check_sessions(data, evidence, errors)
    check_rules(data, errors)
    return errors
