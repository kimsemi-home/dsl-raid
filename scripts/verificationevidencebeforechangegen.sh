#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-evidence-before-change.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-evidence-before-change-json))' |
    python3 -m json.tool
}

validate_evidence_before_change() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("changes", []):
    if row["id"] in seen: errors.append(f"duplicate change {row['id']}")
    seen.add(row["id"])
    if row["change_kind"] not in {"routine", "emergency"}:
        errors.append(f"bad change kind {row['id']}")
    if not row["proposed_by"].startswith("agent:"):
        errors.append(f"bad proposer {row['id']}")
    if row["authority"].startswith("agent:"):
        errors.append(f"agent authority {row['id']}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"unknown evidence {row['id']} {item}")
    if row["change_kind"] != "emergency" and not row.get("evidence"):
        errors.append(f"routine change without evidence {row['id']}")
    if row["change_kind"] == "emergency" and not row.get("evidence"):
        if not row.get("debt"): errors.append(f"emergency without debt {row['id']}")
if not data.get("changes"): errors.append("evidence-before-change has no changes")
if not data.get("closure_rules"): errors.append("evidence-before-change has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification evidence-before-change check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification evidence-before-change is stale: run scripts/verificationevidencebeforechangegen.sh generate" \
      "verification evidence-before-change generated output ok"
    validate_evidence_before_change ;;
  *) echo "usage: scripts/verificationevidencebeforechangegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
