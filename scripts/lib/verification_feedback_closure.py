from verification_feedback_evidence import check_evidence_refs
from verification_feedback_policy import allowed_status, allowed_update_prefixes


def check_closure(row, outputs, seen, closures, errors):
    rid = row.get("id", "")
    if rid in seen:
        errors.append(f"duplicate closure {rid}")
    seen.add(rid)
    closures.add(rid)
    check_identity(row, errors)
    check_status(row, errors)
    check_update(row, errors)
    check_evidence_refs(row, outputs, errors)


def check_identity(row, errors):
    rid = row.get("id", "")
    if not rid.startswith("feedback:"):
        errors.append(f"{rid} must use feedback namespace")
    if not row.get("trigger"):
        errors.append(f"{rid} missing trigger")
    if not row.get("meaning"):
        errors.append(f"{rid} missing meaning")


def check_status(row, errors):
    rid = row.get("id", "")
    if row.get("status") not in allowed_status:
        errors.append(f"bad status {rid}")
    if not row.get("owner") or not row.get("revalidation"):
        errors.append(f"missing owner or revalidation {rid}")


def check_update(row, errors):
    rid = row.get("id", "")
    update = row.get("update", "")
    if row.get("status") == "closed" and not update:
        errors.append(f"missing update {rid}")
    if update and not update.startswith(allowed_update_prefixes):
        errors.append(f"{rid} update is not knowledge-bearing")
