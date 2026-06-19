#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-review-capacity.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-review-capacity-json))' |
    python3 -m json.tool
}

validate_review() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("queues", []):
    if row["id"] in seen: errors.append(f"duplicate queue {row['id']}")
    seen.add(row["id"])
    if row["assigned"] > row["capacity"]: errors.append(f"over capacity {row['id']}")
    if row["status"] not in {"available", "near-limit", "overloaded"}:
        errors.append(f"bad status {row['id']}")
    if row["status"] == "overloaded" and not row.get("freezes"):
        errors.append(f"missing freezes {row['id']}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"unknown evidence {row['id']} {item}")
effects = {row.get("effect") for row in data.get("overload_rules", [])}
if "freeze-governed-automation" not in effects: errors.append("missing freeze rule")
if not data.get("queues"): errors.append("review capacity has no queues")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification review capacity check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification review capacity is stale: run scripts/verificationreviewgen.sh generate" \
      "verification review capacity generated output ok"
    validate_review ;;
  *) echo "usage: scripts/verificationreviewgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
