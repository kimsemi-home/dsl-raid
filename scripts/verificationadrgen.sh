#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-adr-governance.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-adr-json))' |
    python3 -m json.tool
}

validate_adr() {
  python3 - "$repo" "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
repo, path, evidence_path = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
decisions = {"adr-required", "autonomous-allowed"}
errors, seen = [], set()
for row in data.get("records", []):
    if row["id"] in seen: errors.append(f"duplicate adr row {row['id']}")
    seen.add(row["id"])
    if row["decision"] not in decisions:
        errors.append(f"{row['id']} bad decision")
    if row["change_kind"] == "semantic-contract" and row["decision"] != "adr-required":
        errors.append(f"{row['id']} semantic contract is not ADR-gated")
    if row["change_kind"] == "implementation-detail" and row["decision"] == "adr-required":
        errors.append(f"{row['id']} implementation detail over-gated")
    if not (repo / row["policy_doc"]).exists():
        errors.append(f"{row['id']} missing policy doc {row['policy_doc']}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("records"): errors.append("ADR governance manifest has no records")
if not data.get("closure_rules"): errors.append("ADR governance manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification ADR governance check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated ADR governance is stale: run scripts/verificationadrgen.sh generate" \
      "verification ADR governance generated output ok"
    validate_adr ;;
  *) echo "usage: scripts/verificationadrgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
