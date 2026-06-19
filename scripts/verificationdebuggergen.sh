#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-semantic-debugger.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-semantic-debugger-json))' |
    python3 -m json.tool
}

validate_debugger() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("sessions", []):
    if row["id"] in seen:
        errors.append(f"duplicate session {row['id']}")
    seen.add(row["id"])
    if not row.get("inputs"):
        errors.append(f"{row['id']} missing inputs")
    if not row.get("possible_causes"):
        errors.append(f"{row['id']} missing possible causes")
    if not row.get("required_verification"):
        errors.append(f"{row['id']} missing verification")
    if row.get("evidence_quality_risk") not in {"low", "medium", "high"}:
        errors.append(f"{row['id']} bad evidence quality risk")
    if row.get("confidence_ceiling") not in {"low", "medium", "high"}:
        errors.append(f"{row['id']} bad confidence ceiling")
    if row.get("missing_evidence") and row["confidence_ceiling"] == "high":
        errors.append(f"{row['id']} missing evidence cannot permit high confidence")
    if row.get("escalation") not in {"none", "review", "authority-gate"}:
        errors.append(f"{row['id']} bad escalation")
    if row.get("evidence_quality_risk") != "low" and row["escalation"] == "none":
        errors.append(f"{row['id']} risk requires escalation")
    for field in ("inputs", "required_verification", "evidence"):
        for item in row.get(field, []):
            if item not in outputs:
                errors.append(f"{row['id']} unknown {field} {item}")
if not data.get("sessions"):
    errors.append("semantic debugger manifest has no sessions")
if not data.get("closure_rules"):
    errors.append("semantic debugger manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification semantic debugger check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated semantic debugger is stale: run scripts/verificationdebuggergen.sh generate" \
      "verification semantic debugger generated output ok"
    validate_debugger ;;
  *) echo "usage: scripts/verificationdebuggergen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
