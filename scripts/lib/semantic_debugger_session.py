from semantic_debugger_evidence import check_output_refs
from semantic_debugger_policy import escalations, risk_levels


def check_session(row, outputs, seen, errors):
    rid = row.get("id", "")
    if rid in seen:
        errors.append(f"duplicate session {rid}")
    seen.add(rid)
    check_required_fields(row, errors)
    check_risk(row, errors)
    check_output_refs(row, outputs, errors)


def check_required_fields(row, errors):
    rid = row.get("id", "")
    required = (
        ("inputs", "missing inputs"),
        ("possible_causes", "missing possible causes"),
        ("required_verification", "missing verification"),
    )
    for field, message in required:
        if not row.get(field):
            errors.append(f"{rid} {message}")


def check_risk(row, errors):
    rid = row.get("id", "")
    if row.get("evidence_quality_risk") not in risk_levels:
        errors.append(f"{rid} bad evidence quality risk")
    if row.get("confidence_ceiling") not in risk_levels:
        errors.append(f"{rid} bad confidence ceiling")
    if row.get("missing_evidence") and row.get("confidence_ceiling") == "high":
        errors.append(f"{rid} missing evidence cannot permit high confidence")
    if row.get("escalation") not in escalations:
        errors.append(f"{rid} bad escalation")
    if row.get("evidence_quality_risk") != "low" and row.get("escalation") == "none":
        errors.append(f"{rid} risk requires escalation")
