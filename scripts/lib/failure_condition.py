def generated_outputs(evidence):
    return {row["output"] for row in evidence["generated_backends"]}


def check_condition(row, outputs, seen, ordered, errors):
    cid = row.get("id", "")
    if cid in seen:
        errors.append(f"duplicate condition {cid}")
    seen.add(cid)
    ordered.append(cid)
    check_identity(row, errors)
    check_authority(row, errors)
    check_evidence(row, outputs, errors)


def check_identity(row, errors):
    cid = row.get("id", "")
    if not cid.startswith("failure:"):
        errors.append(f"{cid} must use failure namespace")
    if not row.get("signal"):
        errors.append(f"{cid} missing signal")
    if not row.get("meaning"):
        errors.append(f"{cid} missing meaning")


def check_authority(row, errors):
    cid = row.get("id", "")
    if not row.get("blocks"):
        errors.append(f"{cid} missing blocks")
    if row.get("owner", "").startswith("agent:"):
        errors.append(f"{cid} owner cannot be an agent")
    if not row.get("owner", "").startswith("owner:"):
        errors.append(f"{cid} owner must use owner namespace")
    if not row.get("response", "").startswith("response:"):
        errors.append(f"{cid} missing response")


def check_evidence(row, outputs, errors):
    cid = row.get("id", "")
    if not row.get("evidence"):
        errors.append(f"{cid} missing evidence")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{cid} unknown evidence {item}")
