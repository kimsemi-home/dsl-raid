#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-merge-receipt.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-merge-receipt-json))' |
    python3 -m json.tool
}

validate_receipts() {
  python3 - "$repo/$out" <<'PY'
import json, os, sys
data = json.load(open(sys.argv[1]))
errors, seen = [], set()
required = {"head-sync", "required-workflows", "pages-health"}
allowed = ("git status -sb", "gh run list", "curl -L -s")
kinds = {r.get("kind") for r in data.get("receipts", [])}
for row in data.get("receipts", []):
    rid = row.get("id")
    if rid in seen:
        errors.append(f"duplicate merge receipt {rid}")
    seen.add(rid)
    if row.get("status") != "closed":
        errors.append(f"{rid} must be closed")
    if not row.get("command", "").startswith(allowed):
        errors.append(f"{rid} uses unbounded command {row.get('command')}")
    if not row.get("fields"):
        errors.append(f"{rid} must declare decisive fields")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{rid} missing evidence {item}")
if required - kinds:
    errors.append(f"missing merge receipt kinds {sorted(required - kinds)}")
branch = json.load(open("docs/generated/verification-branch-protection.json"))
checks = {r["name"] for r in branch.get("requirements", []) if r["kind"] == "required-check"}
if {"CI", "Security", "Golden", "Verification Graph"} - checks:
    errors.append("branch protection receipts miss required checks")
actions = json.load(open("docs/generated/verification-github-actions.json"))
roles = {w["role"] for w in actions.get("workflows", [])}
if {"generated-ci", "generated-security", "generated-golden", "generated-pages", "generated-graph"} - roles:
    errors.append("generated workflow suite is incomplete")
if not any(r["kind"] == "pages-health" and r["expected"] == "200" for r in data["receipts"]):
    errors.append("pages health receipt must close on HTTP 200")
if not data.get("closure_rules"):
    errors.append("merge receipt manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification merge receipt check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated merge receipt is stale: run scripts/verificationmergereceiptgen.sh generate" \
      "verification merge receipt generated output ok"
    validate_receipts ;;
  *) echo "usage: scripts/verificationmergereceiptgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
