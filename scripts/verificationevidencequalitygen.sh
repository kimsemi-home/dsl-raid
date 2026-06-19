#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-evidence-quality.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-evidence-quality-json))' |
    python3 -m json.tool
}

validate_quality() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("assessments", []):
    if row["id"] in seen: errors.append(f"duplicate assessment {row['id']}")
    seen.add(row["id"])
    if row["target"] not in outputs: errors.append(f"unknown target {row['id']}")
    if row["quality"] not in {"high", "medium", "low"}:
        errors.append(f"bad quality {row['id']}")
    if row["assessed_by"].startswith(("agent:", "control-plane:")):
        errors.append(f"non-governance assessor {row['id']}")
    if not row.get("signals"): errors.append(f"empty signals {row['id']}")
if not data.get("assessments"): errors.append("evidence quality has no assessments")
if not data.get("closure_rules"): errors.append("evidence quality has no closure rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification evidence quality check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification evidence quality is stale: run scripts/verificationevidencequalitygen.sh generate" \
      "verification evidence quality generated output ok"
    validate_quality ;;
  *) echo "usage: scripts/verificationevidencequalitygen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
