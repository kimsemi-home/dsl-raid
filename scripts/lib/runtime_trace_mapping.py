from runtime_trace_paths import exists
from runtime_trace_policy import coverage_statuses


def check_mapping(row, repo, outputs, seen, triples, errors):
    rid = row.get("id", "")
    if rid in seen:
        errors.append(f"duplicate runtime trace {rid}")
    seen.add(rid)
    check_files(row, repo, errors)
    check_subjects(row, errors)
    check_evidence(row, outputs, errors)
    triples.add((row.get("design_ir", ""), row.get("trace", ""), row.get("coverage", "")))


def check_files(row, repo, errors):
    rid = row.get("id", "")
    for key in ("design_ir", "trace", "coverage"):
        if not exists(repo, row.get(key, "")):
            errors.append(f"{rid} missing {key}")


def check_subjects(row, errors):
    rid = row.get("id", "")
    if row.get("runtime_subject") != row.get("design_subject"):
        errors.append(f"{rid} runtime subject differs from design subject")
    if row.get("coverage_status") not in coverage_statuses:
        errors.append(f"{rid} bad coverage status")


def check_evidence(row, outputs, errors):
    rid = row.get("id", "")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{rid} unknown evidence {item}")
