#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-merge-automation.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-merge-automation-json))' |
    python3 -m json.tool
}

validate_merge_automation() {
  python3 - "$repo/$out" <<'PY'
import json, os, pathlib, sys
data = json.load(open(sys.argv[1]))
root = pathlib.Path(".github/workflows")
texts = {p: p.read_text() for p in sorted(root.glob("*.yml"))}
errors, seen = [], set()
policies = {row.get("policy"): row for row in data.get("policies", [])}
for row in data.get("policies", []):
    if row["id"] in seen:
        errors.append(f"duplicate merge automation policy {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "required":
        errors.append(f"{row['id']} must be required")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
if "pull_request_target" in "\n".join(texts.values()):
    errors.append("pull_request_target is forbidden for merge automation")
ci = pathlib.Path(".github/workflows/ci.yml").read_text()
if "pull_request:" not in ci or "contents: read" not in ci:
    errors.append("CI must run on pull_request with contents: read")
readiness = pathlib.Path("docs/generated/verification-merge-readiness.json").read_text()
for name in ("quality", "golden", "privacy", "line-budget", "pages"):
    if name not in readiness:
        errors.append(f"merge readiness missing gate {name}")
if {"source", "required-checks", "forbidden-event", "permission"} - set(policies):
    errors.append("merge automation missing required policy class")
if len(data.get("policies", [])) < 5:
    errors.append("merge automation needs five policies")
if not data.get("closure_rules"):
    errors.append("merge automation manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification merge automation check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated merge automation is stale: run scripts/verificationautomergegen.sh generate" \
      "verification merge automation generated output ok"
    validate_merge_automation ;;
  *) echo "usage: scripts/verificationautomergegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
