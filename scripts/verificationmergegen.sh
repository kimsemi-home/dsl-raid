#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-merge-readiness.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-merge-readiness-json))' |
    python3 -m json.tool
}

validate_merge_readiness() {
  python3 - "$repo/$out" <<'PY'
import json, os, sys
data = json.load(open(sys.argv[1]))
errors, seen = [], set()
commands = {row.get("check") for row in data.get("gates", [])}
required = {
    "cargo run -p dslraid-cli -- quality",
    "cargo run -p dslraid-cli -- golden check tests/golden",
    "bash scripts/privacycheck.sh check",
    "bash scripts/check-source-lines.sh",
    "bash scripts/workflowgen.sh check",
}
for row in data.get("gates", []):
    if row["id"] in seen:
        errors.append(f"duplicate merge gate {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "required":
        errors.append(f"{row['id']} must be required")
    if not row.get("evidence"):
        errors.append(f"{row['id']} has no evidence")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
if not required.issubset(commands):
    errors.append("merge readiness missing required gate command")
if len(data.get("gates", [])) < 5:
    errors.append("merge readiness needs five gates")
if not data.get("closure_rules"):
    errors.append("merge readiness manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification merge readiness check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated merge readiness is stale: run scripts/verificationmergegen.sh generate" \
      "verification merge readiness generated output ok"
    validate_merge_readiness ;;
  *) echo "usage: scripts/verificationmergegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
