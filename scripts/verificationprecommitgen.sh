#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-precommit-closure.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-precommit-json))' |
    python3 -m json.tool
}

validate_precommit() {
  python3 - "$repo/$out" "$repo" <<'PY'
import json, os, pathlib, sys
data = json.load(open(sys.argv[1]))
repo = pathlib.Path(sys.argv[2])
hook = repo / data["hook"]
install = repo / data["install_script"]
errors = []
home_prefix = "/" + "Users" + "/"
raw = pathlib.Path(sys.argv[1]).read_text()
if home_prefix in raw:
    errors.append("precommit manifest leaked a private local path")
if not hook.exists() or not os.access(hook, os.X_OK):
    errors.append("pre-commit hook is missing or not executable")
if not install.exists() or not os.access(install, os.X_OK):
    errors.append("install-hooks script is missing or not executable")
hook_text = hook.read_text() if hook.exists() else ""
install_text = install.read_text() if install.exists() else ""
if "core.hooksPath" not in install_text or ".githooks" not in install_text:
    errors.append("install-hooks must set core.hooksPath=.githooks")
kinds = {row["kind"] for row in data.get("commands", [])}
required = {"go-lint","rust-format","rust-clippy","rust-test","viewer-lint",
            "viewer-test","viewer-build","quality","diff-check"}
if required - kinds:
    errors.append(f"missing precommit command kinds {sorted(required - kinds)}")
for row in data.get("commands", []):
    command = row["command"]
    if command not in hook_text:
        errors.append(f"{row['id']} not enforced by .githooks/pre-commit")
if "cargo run -p dslraid-cli -- quality" not in hook_text:
    errors.append("pre-commit hook must run dslraid quality")
if not data.get("closure_rules"):
    errors.append("precommit closure has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification precommit closure check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification precommit closure is stale: run scripts/verificationprecommitgen.sh generate" \
      "verification precommit closure generated output ok"
    validate_precommit ;;
  *) echo "usage: scripts/verificationprecommitgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
