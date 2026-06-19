#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-evidence-ops.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-evidence-ops-json))' |
    python3 -m json.tool
}

validate_evidence_ops() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
ops = {"ci", "quality-gate", "release", "experiment", "incident"}
claims = {"evidence-generator", "conformance-claim", "pdca-experiment", "knowledge-update"}
effects = {"evidence-only", "review-required", "release-gated", "learning-loop"}
errors, seen = [], set()
for row in data.get("records", []):
    if row["id"] in seen:
        errors.append(f"duplicate evidence ops record {row['id']}")
    seen.add(row["id"])
    if row["operation"] not in ops or row["claim"] not in claims:
        errors.append(f"{row['id']} bad operation or claim")
    if row["authority_effect"] not in effects:
        errors.append(f"{row['id']} bad authority effect")
    if row["owner"].startswith("agent:"):
        errors.append(f"{row['id']} owner cannot be agent")
    if row["operation"] == "ci" and row["authority_effect"] != "evidence-only":
        errors.append(f"{row['id']} CI must be evidence-only")
    if row["operation"] == "release" and row["claim"] != "conformance-claim":
        errors.append(f"{row['id']} release must be conformance claim")
    if row["operation"] == "experiment" and row["authority_effect"] != "review-required":
        errors.append(f"{row['id']} experiment must require review")
    for item in row.get("evidence", []) + row.get("updates", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown generated evidence {item}")
if not data.get("records"):
    errors.append("evidence ops manifest has no records")
if not data.get("closure_rules"):
    errors.append("evidence ops manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification evidence ops check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated evidence ops is stale: run scripts/verificationevidenceopsgen.sh generate" \
      "verification evidence ops generated output ok"
    validate_evidence_ops ;;
  *) echo "usage: scripts/verificationevidenceopsgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
