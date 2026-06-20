def generated_outputs(evidence):
    return {row["output"] for row in evidence["generated_backends"]}


def check_change(row, outputs, seen, changes, errors):
    cid = row.get("id", "")
    if cid in seen:
        errors.append(f"duplicate change {cid}")
    seen.add(cid)
    changes.append((cid, row.get("change_kind", "")))
    check_names(row, errors)
    check_authority(row, errors)
    check_evidence(row, outputs, errors)
    check_debt(row, errors)


def check_names(row, errors):
    cid = row.get("id", "")
    if not cid.startswith("change:"):
        errors.append(f"{cid} must use change namespace")
    if row.get("change_kind") not in {"routine", "emergency"}:
        errors.append(f"bad change kind {cid}")


def check_authority(row, errors):
    cid = row.get("id", "")
    if not row.get("proposed_by", "").startswith("agent:"):
        errors.append(f"bad proposer {cid}")
    if row.get("authority", "").startswith("agent:"):
        errors.append(f"agent authority {cid}")
    if not row.get("authority", "").startswith("gate:"):
        errors.append(f"{cid} authority must use gate namespace")


def check_evidence(row, outputs, errors):
    cid = row.get("id", "")
    evidence = row.get("evidence", [])
    for item in evidence:
        if item not in outputs:
            errors.append(f"unknown evidence {cid} {item}")
    if row.get("change_kind") == "routine" and not evidence:
        errors.append(f"routine change without evidence {cid}")


def check_debt(row, errors):
    cid = row.get("id", "")
    debt = row.get("debt", [])
    for item in debt:
        if not item.startswith("debt:"):
            errors.append(f"{cid} has non-debt marker {item}")
    if row.get("change_kind") == "emergency" and not row.get("evidence") and not debt:
        errors.append(f"emergency without debt {cid}")
