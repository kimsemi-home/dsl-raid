from learning_loop_policy import statuses


def check_cycle(row, stages, outputs, errors):
    cid = row.get("id", "")
    if row.get("status") not in statuses:
        errors.append(f"{cid} bad status")
    if row.get("owner", "").startswith("agent:"):
        errors.append(f"{cid} owner cannot be an agent")
    check_refs(row, stages, outputs, errors)
    check_authority(row, errors)


def check_refs(row, stages, outputs, errors):
    cid = row.get("id", "")
    for stage in row.get("stages", []):
        if stage not in stages:
            errors.append(f"{cid} unknown stage {stage}")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{cid} unknown evidence {item}")


def check_authority(row, errors):
    cid = row.get("id", "")
    if not row.get("knowledge_update", "").startswith("update:"):
        errors.append(f"{cid} missing knowledge update")
    if not row.get("revalidation", "").startswith("revalidate:"):
        errors.append(f"{cid} missing revalidation")
    if not row.get("trigger", "").startswith("trigger:"):
        errors.append(f"{cid} missing trigger")
    if not row.get("incident", "").startswith("incident:"):
        errors.append(f"{cid} missing incident")
