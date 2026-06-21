from verification_sidecar_evidence import (
    check_generated_reference,
    check_output_reference,
)


def check_receipt(row, outputs, seen, receipts, errors):
    rid = row.get("id", "")
    if rid in seen:
        errors.append(f"duplicate receipt {rid}")
    seen.add(rid)
    receipts.add(rid)
    check_identity(row, errors)
    check_separation(row, errors)
    check_output_reference(row, outputs, errors)
    check_evidence(row, outputs, errors)


def check_identity(row, errors):
    rid = row.get("id", "")
    if not rid.startswith("sidecar:"):
        errors.append(f"{rid} must use sidecar namespace")
    if not row.get("meaning"):
        errors.append(f"{rid} missing meaning")


def check_separation(row, errors):
    rid = row.get("id", "")
    if row.get("producer") == row.get("verifier"):
        errors.append(f"producer verifies itself {rid}")
    if row.get("independent") is not True:
        errors.append(f"not independent {rid}")


def check_evidence(row, outputs, errors):
    rid = row.get("id", "")
    if not row.get("evidence"):
        errors.append(f"{rid} must cite evidence")
    for item in row.get("evidence", []):
        check_generated_reference(rid, item, outputs, errors)
