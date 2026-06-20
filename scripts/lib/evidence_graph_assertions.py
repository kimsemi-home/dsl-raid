from evidence_graph_edge import check_edge
from evidence_graph_node import check_node
from evidence_graph_paths import has_private_path
from evidence_graph_policy import (
    generated_by,
    profile,
    required_relations,
    required_rules,
    source,
    subject,
)


def check_manifest(data, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("evidence graph generator mismatch")
    if data.get("evidence_graph_profile") != profile:
        errors.append("evidence graph profile mismatch")
    if data.get("subject") != subject:
        errors.append("evidence graph subject mismatch")
    if data.get("source") != source:
        errors.append("evidence graph source mismatch")
    if has_private_path(text):
        errors.append("evidence graph leaked a private local path")


def check_nodes(data, repo, errors):
    nodes = set()
    for row in data.get("nodes", []):
        check_node(row, repo, nodes, errors)
    if not data.get("nodes"):
        errors.append("evidence graph manifest has no nodes")
    return nodes


def check_edges(data, repo, nodes, errors):
    edges, relations = set(), set()
    for row in data.get("edges", []):
        check_edge(row, repo, nodes, edges, relations, errors)
    if required_relations - relations:
        errors.append(f"missing evidence graph relations {sorted(required_relations - relations)}")
    if "updates" not in relations:
        errors.append("evidence graph must close through feedback")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"evidence graph rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check evidence graph")


def collect_errors(data, repo, text):
    errors = []
    check_manifest(data, text, errors)
    nodes = check_nodes(data, repo, errors)
    check_edges(data, repo, nodes, errors)
    check_rules(data, errors)
    return errors
