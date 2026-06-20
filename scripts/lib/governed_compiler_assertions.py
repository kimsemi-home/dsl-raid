import os

from governed_compiler_policy import (
    generated_by,
    profile,
    required_rules,
    required_stages,
    source,
    subject,
)
from governed_compiler_stage import check_stage


def exists(repo, path):
    return os.path.exists(os.path.join(repo, path))


def check_manifest(data, repo, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("governed compiler generator mismatch")
    if data.get("compiler_farm_profile") != profile:
        errors.append("governed compiler profile mismatch")
    if data.get("subject") != subject:
        errors.append("governed compiler subject mismatch")
    if data.get("source") != source:
        errors.append("governed compiler source mismatch")
    if not exists(repo, source):
        errors.append(f"governed compiler missing source {source}")
    if "/" + "Users" + "/" in text:
        errors.append("governed compiler leaked a private local path")


def check_tracker(tracker, errors):
    if tracker["stages"] != required_stages:
        errors.append("governed compiler stages are not canonical")
    if tracker["orders"] != list(range(1, len(tracker["orders"]) + 1)):
        errors.append("stage order is not contiguous")
    if tracker["trusts"][1:2] != ["candidate"]:
        errors.append("agent output must remain candidate trust")
    if tracker["trusts"][-1:] != ["gated"]:
        errors.append("authority must be the final gated stage")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"governed compiler rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check compiler farm")


def collect_errors(data, repo, text):
    errors, seen = [], set()
    tracker = {"stages": [], "trusts": [], "orders": []}
    check_manifest(data, repo, text, errors)
    for row in data.get("stages", []):
        check_stage(row, repo, seen, tracker, errors)
    if not data.get("stages"):
        errors.append("governed compiler has no stages")
    check_tracker(tracker, errors)
    check_rules(data, errors)
    return errors
