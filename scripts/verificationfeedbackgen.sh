#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-feedback.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-feedback-json))' |
    python3 -m json.tool
}

validate_feedback() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("closures", []):
    if row["id"] in seen: errors.append(f"duplicate closure {row['id']}")
    seen.add(row["id"])
    if row["status"] not in {"open", "closed", "revalidating"}:
        errors.append(f"bad status {row['id']}")
    if row["status"] == "closed" and not row.get("update"):
        errors.append(f"missing update {row['id']}")
    if not row.get("owner") or not row.get("revalidation"):
        errors.append(f"missing owner or revalidation {row['id']}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"unknown evidence {row['id']} {item}")
if not data.get("closures"): errors.append("feedback manifest has no closures")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification feedback closure check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification feedback is stale: run scripts/verificationfeedbackgen.sh generate" \
      "verification feedback generated output ok"
    validate_feedback ;;
  *) echo "usage: scripts/verificationfeedbackgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
