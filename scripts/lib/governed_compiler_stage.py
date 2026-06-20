import os

from governed_compiler_command import check_stage_command


def exists(repo, path):
    return os.path.exists(os.path.join(repo, path))


def check_stage(row, repo, seen, tracker, errors):
    sid = row.get("id", "")
    if sid in seen:
        errors.append(f"duplicate stage {sid}")
    seen.add(sid)
    tracker["stages"].append(row.get("stage", ""))
    tracker["trusts"].append(row.get("trust", ""))
    tracker["orders"].append(row.get("order", 0))
    check_identity(row, errors)
    check_paths(row, repo, errors)
    check_stage_command(row, repo, errors)


def check_identity(row, errors):
    sid = row.get("id", "")
    if not sid.startswith("governed-compiler:"):
        errors.append(f"{sid} must use governed-compiler namespace")
    if not row.get("meaning"):
        errors.append(f"{sid} missing meaning")
    if not row.get("assertion", "").startswith("stdout:"):
        errors.append(f"{sid} assertion must target stdout")


def check_paths(row, repo, errors):
    sid = row.get("id", "")
    for key in ("input", "output"):
        if not exists(repo, row.get(key, "")):
            errors.append(f"{sid} missing {key} {row.get(key, '')}")
    for item in row.get("evidence", []):
        if not exists(repo, item):
            errors.append(f"{sid} missing evidence {item}")
