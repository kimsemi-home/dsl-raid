from learning_loop_cycle import check_cycle
from learning_loop_policy import generated_by, profile, required_rules, required_stages, source, subject
from learning_loop_stage import check_stage, check_stage_order, exists, generated_outputs


def check_manifest(data, repo, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("learning loop generator mismatch")
    if data.get("learning_loop_profile") != profile:
        errors.append("learning loop profile mismatch")
    if data.get("subject") != subject:
        errors.append("learning loop subject mismatch")
    if data.get("source") != source:
        errors.append("learning loop source mismatch")
    if not exists(repo, source):
        errors.append(f"learning loop missing source {source}")
    if "/" + "Users" + "/" in text:
        errors.append("learning loop leaked a private local path")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"learning loop rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check learning loop")


def collect_learning_loop_errors(data, evidence, repo, text):
    errors, seen, orders, stages = [], set(), [], []
    outputs = generated_outputs(evidence)
    check_manifest(data, repo, text, errors)
    for row in data.get("stages", []):
        check_stage(row, outputs, seen, orders, stages, errors)
    if stages != required_stages:
        errors.append("learning stages must follow the canonical path")
    check_stage_order(orders, errors)
    stage_ids = {row[0] for row in stages}
    for row in data.get("cycles", []):
        check_cycle(row, stage_ids, outputs, errors)
    if not data.get("cycles"):
        errors.append("learning loop has no cycles")
    check_rules(data, errors)
    return errors
