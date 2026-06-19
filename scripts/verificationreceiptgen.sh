#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-actions-receipt.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-actions-receipt-json))' |
    python3 -m json.tool
}

validate_receipts() {
  python3 - "$repo/$out" <<'PY'
import json, os, pathlib, sys
data = json.load(open(sys.argv[1]))
errors, seen = [], set()
required_kinds = {"workflow-summary", "head-sha", "job-detail", "pages-health", "forbidden-event"}
required_fields = {"headSha", "status", "conclusion", "url"}
allowed = ("gh run list", "gh run view", "curl -I -L", "grep -R")
kinds = {r.get("kind") for r in data.get("receipts", [])}
for row in data.get("receipts", []):
    if row["id"] in seen:
        errors.append(f"duplicate actions receipt {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "required":
        errors.append(f"{row['id']} must be required")
    if not row.get("command", "").startswith(allowed):
        errors.append(f"{row['id']} uses unbounded command {row.get('command')}")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
if required_kinds - kinds:
    errors.append(f"missing receipt kinds {sorted(required_kinds - kinds)}")
workflow = next(r for r in data["receipts"] if r["kind"] == "workflow-summary")
if required_fields - set(workflow.get("fields", [])):
    errors.append("workflow summary receipt misses remote verdict fields")
if not any(r["kind"] == "pages-health" and "HTTP/2 200" in r["expected"] for r in data["receipts"]):
    errors.append("pages receipt must expect HTTP/2 200")
if any("pull_request_target" in p.read_text() for p in pathlib.Path(".github/workflows").glob("*.yml")):
    errors.append("workflow uses forbidden pull_request_target")
branch = json.load(open("docs/generated/verification-branch-protection.json"))
checks = {r["name"] for r in branch.get("requirements", []) if r["kind"] == "required-check"}
if {"CI", "Security", "Golden", "Verification Graph"} - checks:
    errors.append("branch protection required checks are incomplete")
if not data.get("closure_rules"):
    errors.append("actions receipt manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification actions receipt check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated actions receipt is stale: run scripts/verificationreceiptgen.sh generate" \
      "verification actions receipt generated output ok"
    validate_receipts ;;
  *) echo "usage: scripts/verificationreceiptgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
