from failure_recovery_policy import statuses


def check_recovery(row, context, seen, ordered, errors):
    rid = row.get("id", "")
    if rid in seen:
        errors.append(f"duplicate recovery {rid}")
    seen.add(rid)
    ordered.append(rid)
    check_identity(row, errors)
    check_links(row, context, errors)
    check_status(row, errors)
    check_evidence(row, context, errors)


def check_identity(row, errors):
    rid = row.get("id", "")
    if not rid.startswith("failure-recovery:"):
        errors.append(f"{rid} must use failure-recovery namespace")
    if not row.get("root_cause", "").startswith("root-cause:"):
        errors.append(f"{rid} missing root cause")
    if not row.get("release_gate", "").startswith("gate:"):
        errors.append(f"{rid} bad gate")


def check_links(row, context, errors):
    rid = row.get("id", "")
    failure = row.get("failure", "")
    responses = context["responses"]
    if failure not in responses:
        errors.append(f"{rid} unknown failure {failure}")
    elif row.get("response") != responses[failure]:
        errors.append(f"{rid} response mismatch")
    if row.get("incident") not in context["incidents"]:
        errors.append(f"{rid} unknown incident")
    if row.get("learning_update") not in context["updates"]:
        errors.append(f"{rid} unknown update")
    if row.get("revalidation") not in context["revalidations"]:
        errors.append(f"{rid} unknown revalidation")


def check_status(row, errors):
    rid = row.get("id", "")
    if row.get("status") not in statuses:
        errors.append(f"{rid} bad status")


def check_evidence(row, context, errors):
    rid = row.get("id", "")
    if not row.get("evidence"):
        errors.append(f"{rid} missing evidence")
    for item in row.get("evidence", []):
        if item not in context["outputs"]:
            errors.append(f"{rid} unknown evidence {item}")
