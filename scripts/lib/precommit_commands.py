from precommit_policy import required_kinds


def check_commands(data, hook_text, errors):
    kinds, seen = set(), set()
    for row in data.get("commands", []):
        cid = row.get("id", "")
        if cid in seen:
            errors.append(f"duplicate precommit command {cid}")
        seen.add(cid)
        check_command(row, hook_text, errors)
        kinds.add(row.get("kind", ""))
    missing = required_kinds - kinds
    if missing:
        errors.append(f"missing precommit command kinds {sorted(missing)}")


def check_command(row, hook_text, errors):
    cid = row.get("id", "")
    if not cid.startswith("precommit:"):
        errors.append(f"{cid} must use precommit namespace")
    if not row.get("meaning"):
        errors.append(f"{cid} missing meaning")
    command = row.get("command", "")
    if command not in hook_text:
        errors.append(f"{cid} not enforced by .githooks/pre-commit")
