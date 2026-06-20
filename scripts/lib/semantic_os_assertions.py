import os
import subprocess

from semantic_os_policy import generated_by, profile, required_roles, required_rules, subject


def exists(repo, path):
    return path.startswith("stdout:") or os.path.exists(os.path.join(repo, path))

def run(repo, command):
    return subprocess.run(
        ["bash", "-lc", command],
        cwd=repo,
        text=True,
        capture_output=True,
    )


def check_manifest(data, repo, errors):
    if data.get("generated_by") != generated_by:
        errors.append("semantic os generator mismatch")
    if data.get("semantic_os_profile") != profile:
        errors.append("semantic os profile mismatch")
    if data.get("subject") != subject:
        errors.append("semantic os subject mismatch")
    if not exists(repo, data.get("source", "")):
        errors.append(f"semantic os missing source {data.get('source')}")

def check_layer(row, repo, seen, roles, errors):
    rid = row["id"]
    if rid in seen:
        errors.append(f"duplicate layer {rid}")
    seen.add(rid)
    roles.add(row["role"])
    if rid != f"semantic-os:{row['role']}":
        errors.append(f"{rid} must match role {row['role']}")
    for key in ("source", "artifact"):
        if not exists(repo, row[key]):
            errors.append(f"{rid} missing {key} {row[key]}")
    for item in row.get("evidence", []):
        if not exists(repo, item):
            errors.append(f"{rid} missing evidence {item}")
    check_layer_command(row, repo, errors)

def check_layer_command(row, repo, errors):
    assertion = row.get("assertion", "")
    if not assertion.startswith("stdout:"):
        errors.append(f"{row['id']} assertion must target stdout")
        return
    result = run(repo, row["command"])
    expected = assertion.removeprefix("stdout:")
    if result.returncode or expected not in result.stdout:
        errors.append(f"{row['id']} expected stdout {expected!r}")


def check_closure_rules(data, errors):
    found = {row["id"] for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"semantic os rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row['id']} must self-check semantic os")


def collect_semantic_os_errors(data, repo):
    errors, roles, seen = [], set(), set()
    check_manifest(data, repo, errors)
    for row in data.get("layers", []):
        check_layer(row, repo, seen, roles, errors)
    missing = sorted(required_roles - roles)
    if missing:
        errors.append(f"missing semantic os roles {missing}")
    check_closure_rules(data, errors)
    return errors
