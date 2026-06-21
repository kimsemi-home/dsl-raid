from verification_experiment_decision_paths import exists
from verification_experiment_decision_policy import allowed_decisions, checked_status


def check_decision(row, exp, repo, seen, decisions, errors):
    rid = row.get("id", "")
    if rid in seen:
        errors.append(f"duplicate decision {rid}")
    seen.add(rid)
    decisions.add(rid)
    check_identity(row, errors)
    if exp is None:
        errors.append(f"{rid} references unknown experiment")
        return
    check_closure(row, exp, errors)
    check_files(row, repo, errors)


def check_identity(row, errors):
    rid = row.get("id", "")
    if not rid.startswith("experiment-decision:"):
        errors.append(f"{rid} must use experiment-decision namespace")
    if row.get("decision") not in allowed_decisions:
        errors.append(f"{rid} bad decision")
    if not row.get("gate", "").startswith("gate:"):
        errors.append(f"{rid} gate must be semantic")
    if not row.get("meaning"):
        errors.append(f"{rid} missing meaning")


def check_closure(row, exp, errors):
    rid = row.get("id", "")
    if row.get("act") != exp.get("act"):
        errors.append(f"{rid} act does not match experiment")
    if row.get("decision") == "promote" and not exp.get("promoted"):
        errors.append(f"{rid} promotes an unpromoted experiment")
    if row.get("decision") == "promote" and row.get("status") != "closed":
        errors.append(f"{rid} promotion must be closed")
    if exp.get("status") not in checked_status:
        errors.append(f"{rid} experiment was not checked")


def check_files(row, repo, errors):
    rid = row.get("id", "")
    for item in row.get("evidence", []) + [row.get("act", "")]:
        if not exists(repo, item):
            errors.append(f"{rid} missing evidence {item}")
