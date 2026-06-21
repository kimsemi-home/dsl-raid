from verification_quarantine_bundle import check_bundle
from verification_quarantine_evidence import generated_outputs
from verification_quarantine_paths import has_private_path
from verification_quarantine_policy import (
    generated_by,
    profile,
    required_bundles,
    required_rules,
    source,
    subject,
)


def check_manifest(data, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("quarantine generator mismatch")
    if data.get("quarantine_profile") != profile:
        errors.append("quarantine profile mismatch")
    if data.get("subject") != subject:
        errors.append("quarantine subject mismatch")
    if data.get("source") != source:
        errors.append("quarantine source mismatch")
    if has_private_path(text):
        errors.append("quarantine manifest leaked a private local path")


def check_bundles(data, outputs, errors):
    seen, bundles = set(), set()
    for row in data.get("bundles", []):
        check_bundle(row, outputs, seen, bundles, errors)
    if not data.get("bundles"):
        errors.append("quarantine manifest has no bundles")
    missing = required_bundles - bundles
    if missing:
        errors.append(f"missing quarantine bundles {sorted(missing)}")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if not found:
        errors.append("quarantine manifest has no closure rules")
    if found != required_rules:
        errors.append(f"quarantine rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check quarantine")


def collect_errors(data, evidence, text):
    errors = []
    check_manifest(data, text, errors)
    check_bundles(data, generated_outputs(evidence), errors)
    check_rules(data, errors)
    return errors
