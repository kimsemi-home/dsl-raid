from precommit_commands import check_commands
from precommit_paths import has_private_path, is_executable, read_if_exists, repo_path
from precommit_policy import generated_by, profile, required_rules, source, subject


def check_manifest(data, manifest_text, errors):
    if data.get("generated_by") != generated_by:
        errors.append("precommit generator mismatch")
    if data.get("precommit_profile") != profile:
        errors.append("precommit profile mismatch")
    if data.get("subject") != subject:
        errors.append("precommit subject mismatch")
    if data.get("source") != source:
        errors.append("precommit source mismatch")
    if has_private_path(manifest_text):
        errors.append("precommit manifest leaked a private local path")


def check_files(data, repo, errors):
    hook = repo_path(repo, data.get("hook", ""))
    install = repo_path(repo, data.get("install_script", ""))
    if not is_executable(hook):
        errors.append("pre-commit hook is missing or not executable")
    if not is_executable(install):
        errors.append("install-hooks script is missing or not executable")
    install_text = read_if_exists(install)
    if "core.hooksPath" not in install_text or ".githooks" not in install_text:
        errors.append("install-hooks must set core.hooksPath=.githooks")
    return read_if_exists(hook)


def check_rules(data, errors):
    found = {row.get("id", "") for row in data.get("closure_rules", [])}
    if found != required_rules:
        errors.append(f"precommit rule set mismatch: {sorted(found)}")
    for row in data.get("closure_rules", []):
        if row.get("check") != f"{generated_by} check":
            errors.append(f"{row.get('id', '')} must self-check precommit closure")


def collect_errors(data, repo, manifest_text):
    errors = []
    check_manifest(data, manifest_text, errors)
    hook_text = check_files(data, repo, errors)
    check_commands(data, hook_text, errors)
    if "cargo run -p dslraid-cli -- quality" not in hook_text:
        errors.append("pre-commit hook must run dslraid quality")
    check_rules(data, errors)
    return errors
