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


def check_step(row, repo, seen, orders, steps, errors):
    sid = row.get("id", "")
    if sid in seen:
        errors.append(f"duplicate knowledge conversion step {sid}")
    seen.add(sid)
    orders.append(row.get("order"))
    steps.append((sid, row.get("phase", "")))
    check_paths(row, repo, errors)
    check_identity(row, errors)
    check_command(row, repo, errors)


def check_paths(row, repo, errors):
    sid = row.get("id", "")
    for key in ("input", "output"):
        if not exists(repo, row.get(key, "")):
            errors.append(f"{sid} missing {key} {row.get(key, '')}")
    evidence = row.get("evidence", [])
    if row.get("output") not in evidence:
        errors.append(f"{sid} output must be listed as evidence")
    for item in evidence:
        if not exists(repo, item):
            errors.append(f"{sid} missing evidence {item}")


def check_identity(row, errors):
    sid = row.get("id", "")
    if not sid.startswith("knowledge-conversion:"):
        errors.append(f"{sid} must use knowledge-conversion id")
    if not row.get("gate", "").startswith("gate:"):
        errors.append(f"{sid} gate must use gate namespace")
    if row.get("input") == row.get("output"):
        errors.append(f"{sid} input and output must stay separate")


def check_command(row, repo, errors):
    assertion = row.get("assertion", "")
    if not assertion.startswith("stdout:"):
        errors.append(f"{row.get('id', '')} assertion must target stdout")
        return
    result = run(repo, row.get("command", ""))
    expected = assertion.removeprefix("stdout:")
    if result.returncode or expected not in result.stdout:
        errors.append(f"{row.get('id', '')} expected stdout {expected!r}")
