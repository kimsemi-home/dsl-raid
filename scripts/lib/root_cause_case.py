def generated_outputs(evidence):
    return {row["output"] for row in evidence["generated_backends"]}


def check_case(row, outputs, seen, cases, errors):
    cid = row.get("id", "")
    if cid in seen:
        errors.append(f"duplicate root cause case {cid}")
    seen.add(cid)
    cases.append((cid, row.get("status", "")))
    check_identity(row, errors)
    check_candidates(row, errors)
    check_evidence(row, outputs, errors)
    check_authority(row, errors)


def check_identity(row, errors):
    cid = row.get("id", "")
    if not cid.startswith("root-cause:"):
        errors.append(f"{cid} must use root-cause namespace")
    if not row.get("observation", "").startswith("observation:"):
        errors.append(f"{cid} missing observation namespace")


def check_candidates(row, errors):
    cid = row.get("id", "")
    candidates = row.get("candidates", [])
    eliminated = row.get("eliminated_causes", [])
    if not candidates:
        errors.append(f"{cid} missing candidates")
    for item in candidates + eliminated:
        if not item.startswith("cause:"):
            errors.append(f"{cid} bad cause {item}")
    for item in eliminated:
        if item not in candidates:
            errors.append(f"{cid} eliminated non-candidate {item}")


def check_evidence(row, outputs, errors):
    cid = row.get("id", "")
    if not row.get("validation_evidence"):
        errors.append(f"{cid} missing validation evidence")
    for field in ("validation_evidence", "evidence"):
        for item in row.get(field, []):
            if item not in outputs:
                errors.append(f"{cid} unknown {field} {item}")


def check_authority(row, errors):
    cid = row.get("id", "")
    if row.get("authority", "").startswith("agent:"):
        errors.append(f"{cid} authority cannot be an agent")
    if not row.get("authority", "").startswith("gate:"):
        errors.append(f"{cid} authority must use gate namespace")
