from evidence_graph_paths import exists


def check_node(row, repo, nodes, errors):
    rid = row.get("id", "")
    if rid in nodes:
        errors.append(f"duplicate evidence graph node {rid}")
    nodes.add(rid)
    if not exists(repo, row.get("artifact", "")):
        errors.append(f"{rid} missing artifact {row.get('artifact')}")
    check_evidence(row, repo, errors)


def check_evidence(row, repo, errors):
    rid = row.get("id", "")
    for item in row.get("evidence", []):
        if not exists(repo, item):
            errors.append(f"{rid} missing evidence {item}")
