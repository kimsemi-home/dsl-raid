from runtime_trace_mapping import check_mapping
from runtime_trace_paths import has_private_path
from runtime_trace_policy import (
    generated_by,
    profile,
    required_rules,
    source,
    subject,
)


def generated_outputs(evidence):
    return {row.get("output", "") for row in evidence.get("generated_backends", [])}


def check_manifest(data, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("runtime trace generator mismatch")
    if data.get("runtime_trace_profile") != profile:
        errors.append("runtime trace profile mismatch")
    if data.get("subject") != subject:
        errors.append("runtime trace subject mismatch")
    if data.get("source") != source:
        errors.append("runtime trace source mismatch")
    if has_private_path(text):
        errors.append("runtime trace leaked a private local path")


def check_mappings(data, evidence, repo, errors):
    outputs = generated_outputs(evidence)
    triples, seen = set(), set()
    for row in data.get("mappings", []):
        check_mapping(row, repo, outputs, seen, triples, errors)
    if not data.get("mappings"):
        errors.append("runtime trace manifest has no mappings")
    return triples


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"runtime trace rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check runtime trace")


def collect_result(data, evidence, repo, text):
    errors = []
    check_manifest(data, text, errors)
    triples = check_mappings(data, evidence, repo, errors)
    check_rules(data, errors)
    return errors, triples
