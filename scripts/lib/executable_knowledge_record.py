import os
import subprocess


def exists(repo, path):
    return os.path.exists(os.path.join(repo, path))


def run(repo, command):
    return subprocess.run(
        ["bash", "-lc", command],
        cwd=repo,
        text=True,
        capture_output=True,
    )


def check_record(row, repo, seen, kinds, errors):
    rid, kind = row["id"], row["kind"]
    if rid in seen:
        errors.append(f"duplicate record {rid}")
    seen.add(rid)
    kinds.append(kind)
    if rid != f"executable-knowledge:{kind}":
        errors.append(f"{rid} must match kind {kind}")
    if not exists(repo, row["source"]):
        errors.append(f"{rid} missing source")
    if row["source"] in row.get("generated", []):
        errors.append(f"{rid} source cannot equal generated artifact")
    check_artifacts(row, repo, errors)
    check_command(row, repo, errors)


def check_artifacts(row, repo, errors):
    rid = row["id"]
    for key in ("generated", "evidence"):
        values = row.get(key, [])
        if not values:
            errors.append(f"{rid} must list {key}")
        for item in values:
            if not exists(repo, item):
                errors.append(f"{rid} missing {key} {item}")


def check_command(row, repo, errors):
    assertion = row.get("assertion", "")
    if not assertion.startswith("stdout:"):
        errors.append(f"{row['id']} assertion must target stdout")
        return
    result = run(repo, row["command"])
    expected = assertion.removeprefix("stdout:")
    if result.returncode or expected not in result.stdout:
        errors.append(f"{row['id']} expected stdout {expected!r}")
