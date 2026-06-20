from branch_protection_paths import exists, read_text


def check_requirement(row, repo, seen, checks, targets, errors):
    rid = row.get("id", "")
    if rid in seen:
        errors.append(f"duplicate branch protection requirement {rid}")
    seen.add(rid)
    check_identity(row, errors)
    if row.get("kind") == "required-check":
        checks.add(row.get("name", ""))
    if row.get("kind") == "branch":
        targets.add(row.get("name", ""))
    check_workflow(row, repo, errors)
    check_evidence(row, repo, errors)


def check_identity(row, errors):
    rid = row.get("id", "")
    if not rid.startswith("branch-protect:"):
        errors.append(f"{rid} must use branch-protect namespace")
    if row.get("status") != "required":
        errors.append(f"{rid} must be required")
    if not row.get("meaning"):
        errors.append(f"{rid} missing meaning")


def check_workflow(row, repo, errors):
    rid = row.get("id", "")
    workflow = row.get("workflow", "")
    if not exists(repo, workflow):
        errors.append(f"{rid} missing workflow {workflow}")
        return
    text = read_text(repo, workflow)
    if row.get("kind") == "required-check" and f"name: {row.get('name', '')}" not in text:
        errors.append(f"{rid} missing workflow name {row.get('name', '')}")
    if "pull_request_target" in text:
        errors.append(f"{rid} uses forbidden pull_request_target")


def check_evidence(row, repo, errors):
    rid = row.get("id", "")
    if not row.get("evidence"):
        errors.append(f"{rid} must cite evidence")
    for item in row.get("evidence", []):
        if not exists(repo, item):
            errors.append(f"{rid} missing evidence {item}")
