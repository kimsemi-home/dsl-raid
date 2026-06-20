from knowledge_conversion_policy import (
    generated_by,
    profile,
    required_rules,
    required_steps,
    source,
    subject,
)
from knowledge_conversion_step import check_step, exists


def check_manifest(data, repo, errors):
    if data.get("generated_by") != generated_by:
        errors.append("knowledge conversion generator mismatch")
    if data.get("knowledge_conversion_profile") != profile:
        errors.append("knowledge conversion profile mismatch")
    if data.get("subject") != subject:
        errors.append("knowledge conversion subject mismatch")
    if data.get("source") != source:
        errors.append("knowledge conversion source mismatch")
    if not exists(repo, source):
        errors.append(f"knowledge conversion missing source {source}")


def check_steps(steps, errors):
    if steps != required_steps:
        errors.append("knowledge conversion phases are not canonical")


def check_orders(orders, errors):
    expected = list(range(1, len(orders) + 1))
    if orders != expected:
        errors.append("knowledge conversion order is not contiguous")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"knowledge conversion rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check knowledge conversion")


def collect_knowledge_conversion_errors(data, repo):
    errors, seen, orders, steps = [], set(), [], []
    check_manifest(data, repo, errors)
    for row in data.get("steps", []):
        check_step(row, repo, seen, orders, steps, errors)
    check_steps(steps, errors)
    check_orders(orders, errors)
    check_rules(data, errors)
    return errors
