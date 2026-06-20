from branch_protection_paths import has_private_path
from branch_protection_policy import (
    generated_by,
    profile,
    required_checks,
    required_rules,
    source,
    subject,
)
from branch_protection_requirement import check_requirement


def check_manifest(data, text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("branch protection generator mismatch")
    if data.get("branch_protection_profile") != profile:
        errors.append("branch protection profile mismatch")
    if data.get("subject") != subject:
        errors.append("branch protection subject mismatch")
    if data.get("source") != source:
        errors.append("branch protection source mismatch")
    if has_private_path(text):
        errors.append("branch protection leaked a private local path")


def check_requirements(data, repo, errors):
    seen, checks, targets = set(), set(), set()
    for row in data.get("requirements", []):
        check_requirement(row, repo, seen, checks, targets, errors)
    if required_checks - checks:
        errors.append(f"missing required checks {sorted(required_checks - checks)}")
    if "main" not in targets:
        errors.append("branch protection target must be main")
    if not data.get("requirements"):
        errors.append("branch protection manifest has no requirements")


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"branch protection rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check branch protection")


def collect_errors(data, repo, text):
    errors = []
    check_manifest(data, text, errors)
    check_requirements(data, repo, errors)
    check_rules(data, errors)
    return errors
