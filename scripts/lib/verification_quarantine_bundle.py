from verification_quarantine_evidence import check_evidence_refs
from verification_quarantine_policy import (
    allowed_status,
    allowed_subject_kinds,
    required_blocks,
)


def check_bundle(row, outputs, seen, bundles, errors):
    rid = row.get("id", "")
    if rid in seen:
        errors.append(f"duplicate bundle {rid}")
    seen.add(rid)
    bundles.add(rid)
    check_identity(row, errors)
    check_status(row, errors)
    check_blocks(row, errors)
    check_evidence_refs(row, outputs, errors)


def check_identity(row, errors):
    rid = row.get("id", "")
    if not rid.startswith("quarantine:"):
        errors.append(f"{rid} must use quarantine namespace")
    if row.get("owner") != "gate:quarantine":
        errors.append(f"{rid} must be quarantine-owned")
    if not row.get("meaning"):
        errors.append(f"{rid} missing meaning")


def check_status(row, errors):
    rid = row.get("id", "")
    if row.get("status") not in allowed_status:
        errors.append(f"bad status {rid}")
    if row.get("subject_kind") not in allowed_subject_kinds:
        errors.append(f"bad subject kind {rid}")


def check_blocks(row, errors):
    rid = row.get("id", "")
    missing = required_blocks - set(row.get("blocks", []))
    if missing:
        errors.append(f"missing blocks {rid} {sorted(missing)}")
