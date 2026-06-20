from actions_receipt_paths import exists
from actions_receipt_policy import allowed_prefixes


def check_receipt(row, repo, seen, kinds, errors):
    rid = row.get("id", "")
    if rid in seen:
        errors.append(f"duplicate actions receipt {rid}")
    seen.add(rid)
    kinds.add(row.get("kind", ""))
    check_identity(row, errors)
    check_command(row, errors)
    check_evidence(row, repo, errors)


def check_identity(row, errors):
    rid = row.get("id", "")
    if not rid.startswith("actions-receipt:"):
        errors.append(f"{rid} must use actions-receipt namespace")
    if row.get("status") != "required":
        errors.append(f"{rid} must be required")
    if not row.get("meaning"):
        errors.append(f"{rid} missing meaning")
    if not row.get("fields"):
        errors.append(f"{rid} missing decisive fields")


def check_command(row, errors):
    rid = row.get("id", "")
    command = row.get("command", "")
    if not command.startswith(allowed_prefixes):
        errors.append(f"{rid} uses unbounded command {command}")
    if not row.get("expected"):
        errors.append(f"{rid} missing expected receipt result")


def check_evidence(row, repo, errors):
    rid = row.get("id", "")
    if not row.get("evidence"):
        errors.append(f"{rid} must cite evidence")
    for item in row.get("evidence", []):
        if not exists(repo, item):
            errors.append(f"{rid} missing evidence {item}")
