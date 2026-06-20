from evidence_graph_paths import exists


def check_edge(row, repo, nodes, edges, relations, errors):
    rid = row.get("id", "")
    if rid in edges:
        errors.append(f"duplicate evidence graph edge {rid}")
    edges.add(rid)
    check_endpoints(row, nodes, errors)
    check_status(row, errors)
    relations.add(row.get("relation", ""))
    check_evidence(row, repo, errors)


def check_endpoints(row, nodes, errors):
    rid = row.get("id", "")
    if row.get("from") not in nodes or row.get("to") not in nodes:
        errors.append(f"{rid} references an unknown endpoint")
    if row.get("from") == row.get("to"):
        errors.append(f"{rid} must not be a self loop")


def check_status(row, errors):
    if row.get("status") != "linked":
        errors.append(f"{row.get('id', '')} must be linked")


def check_evidence(row, repo, errors):
    rid = row.get("id", "")
    for item in row.get("evidence", []):
        if not exists(repo, item):
            errors.append(f"{rid} missing evidence {item}")
