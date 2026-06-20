#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-quarantine-release.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-quarantine-release-json))' |
    python3 -m json.tool
}

validate_quarantine_release() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
required = set(data.get("required_conditions", []))
errors, seen = [], set()
for row in data.get("release_gates", []):
    if row["id"] in seen: errors.append(f"duplicate release gate {row['id']}")
    seen.add(row["id"])
    if not row["released_by"].startswith("steward:"):
        errors.append(f"{row['id']} release is not steward-owned")
    if row["verdict"] not in {"confirmed-clean", "partial-release", "debt-open"}:
        errors.append(f"{row['id']} bad verdict")
    conditions = set(row.get("conditions", []))
    if row["verdict"] == "confirmed-clean" and conditions != required:
        errors.append(f"{row['id']} missing confirmed-clean conditions")
    if row["verdict"] == "partial-release":
        if not row.get("reusable") or not row.get("invalidated"):
            errors.append(f"{row['id']} partial release lacks reuse policy")
    if row["verdict"] == "debt-open" and not row.get("debt"):
        errors.append(f"{row['id']} debt verdict lacks debt")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"{row['id']} unknown evidence {item}")
if len(required) < 8: errors.append("release conditions are incomplete")
if not data.get("release_gates"): errors.append("quarantine release has no gates")
if not data.get("closure_rules"): errors.append("quarantine release has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification quarantine release check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated quarantine release is stale: run scripts/verificationquarantinereleasegen.sh generate" \
      "verification quarantine release generated output ok"
    validate_quarantine_release ;;
  *) echo "usage: scripts/verificationquarantinereleasegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
