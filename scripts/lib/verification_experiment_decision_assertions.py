from verification_experiment_decision_lookup import (
    check_promoted_experiments,
    decision_records,
    experiment_records,
)
from verification_experiment_decision_paths import has_private_path
from verification_experiment_decision_policy import (
    generated_by,
    profile,
    required_decisions,
    required_rules,
    source,
    subject,
)
from verification_experiment_decision_record import check_decision


def check_manifest(data, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("experiment decision generator mismatch")
    if data.get("experiment_decision_profile") != profile:
        errors.append("experiment decision profile mismatch")
    if data.get("subject") != subject:
        errors.append("experiment decision subject mismatch")
    if data.get("source") != source:
        errors.append("experiment decision source mismatch")
    if has_private_path(text):
        errors.append("experiment decision leaked a private local path")


def check_decisions(data, experiments, repo, errors):
    records = experiment_records(experiments)
    by_experiment = decision_records(data)
    seen, decisions = set(), set()
    for row in data.get("decisions", []):
        check_decision(row, records.get(row.get("experiment", "")), repo, seen, decisions, errors)
    if not data.get("decisions"):
        errors.append("experiment decision has no decisions")
    missing = required_decisions - decisions
    if missing:
        errors.append(f"missing experiment decisions {sorted(missing)}")
    check_promoted_experiments(records, by_experiment, errors)


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if not found:
        errors.append("experiment decision has no rules")
    if found != required_rules:
        errors.append(f"experiment decision rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check decisions")


def collect_errors(data, experiments, repo, text):
    errors = []
    check_manifest(data, text, errors)
    check_decisions(data, experiments, repo, errors)
    check_rules(data, errors)
    return errors
