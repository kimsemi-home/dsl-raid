#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-branch-protection.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-branch-protection-json))' |
    python3 -m json.tool
}

validate_branch_protection() {
  python3 - "$repo/$out" <<'PY'
import json, os, pathlib, sys
data = json.load(open(sys.argv[1]))
errors, seen = [], set()
required = {"CI", "Security", "Golden", "Verification Graph"}
checks = {r["name"] for r in data.get("requirements", []) if r["kind"] == "required-check"}
for row in data.get("requirements", []):
    if row["id"] in seen:
        errors.append(f"duplicate branch protection requirement {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "required":
        errors.append(f"{row['id']} must be required")
    workflow = pathlib.Path(row["workflow"])
    if not workflow.exists():
        errors.append(f"{row['id']} missing workflow {workflow}")
        continue
    text = workflow.read_text()
    if row["kind"] == "required-check" and f"name: {row['name']}" not in text:
        errors.append(f"{row['id']} missing workflow name {row['name']}")
    if "pull_request_target" in text:
        errors.append(f"{row['id']} uses forbidden pull_request_target")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
if required - checks:
    errors.append(f"missing required checks {sorted(required - checks)}")
if not any(r["kind"] == "branch" and r["name"] == "main" for r in data.get("requirements", [])):
    errors.append("branch protection target must be main")
if not data.get("closure_rules"):
    errors.append("branch protection manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification branch protection check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated branch protection is stale: run scripts/verificationbranchgen.sh generate" \
      "verification branch protection generated output ok"
    validate_branch_protection ;;
  *) echo "usage: scripts/verificationbranchgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
