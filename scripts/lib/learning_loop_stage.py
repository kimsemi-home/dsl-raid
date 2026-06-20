import os


def exists(repo, path):
    return os.path.exists(os.path.join(repo, path))


def generated_outputs(evidence):
    return {row["output"] for row in evidence["generated_backends"]}


def check_stage(row, outputs, seen, orders, stages, errors):
    sid = row.get("id", "")
    if sid in seen:
        errors.append(f"duplicate learning stage {sid}")
    seen.add(sid)
    orders.append(row.get("order"))
    stages.append((sid, row.get("from", ""), row.get("to", "")))
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{sid} unknown evidence {item}")
    if not sid.startswith("stage:"):
        errors.append(f"{sid} must use stage namespace")


def check_stage_order(orders, errors):
    expected = list(range(1, len(orders) + 1))
    if orders != expected:
        errors.append("learning stages must be contiguous")
