from verification_sidecar_evidence import generated_outputs
from verification_sidecar_paths import has_private_path
from verification_sidecar_policy import (
    generated_by,
    profile,
    required_receipts,
    required_rules,
    source,
    subject,
)
from verification_sidecar_receipt import check_receipt


def check_manifest(data, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("sidecar generator mismatch")
    if data.get("sidecar_profile") != profile:
        errors.append("sidecar profile mismatch")
    if data.get("subject") != subject:
        errors.append("sidecar subject mismatch")
    if data.get("source") != source:
        errors.append("sidecar source mismatch")
    if has_private_path(text):
        errors.append("sidecar manifest leaked a private local path")


def check_receipts(data, outputs, errors):
    seen, receipts = set(), set()
    for row in data.get("receipts", []):
        check_receipt(row, outputs, seen, receipts, errors)
    if not data.get("receipts"):
        errors.append("sidecar manifest has no receipts")
    missing = required_receipts - receipts
    if missing:
        errors.append(f"missing sidecar receipts {sorted(missing)}")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if not found:
        errors.append("sidecar manifest has no closure rules")
    if found != required_rules:
        errors.append(f"sidecar rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check sidecar")


def collect_errors(data, evidence, text):
    errors = []
    check_manifest(data, text, errors)
    check_receipts(data, generated_outputs(evidence), errors)
    check_rules(data, errors)
    return errors
