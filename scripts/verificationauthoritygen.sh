#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-authority.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-authority-json))' |
    python3 -m json.tool
}

validate_authority() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("decisions", []):
    if row["id"] in seen: errors.append(f"duplicate decision {row['id']}")
    seen.add(row["id"])
    if row["approved_by"].startswith(("agent:", "control-plane:")):
        errors.append(f"non-governance approver {row['id']}")
    if row["decision"] not in {"approved", "escalated", "rejected"}:
        errors.append(f"bad decision {row['id']}")
    if not row.get("requires"): errors.append(f"empty requires {row['id']}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"unknown evidence {row['id']} {item}")
markers = ["/" + "Users" + "/", "gh" + "o_", "github" + "_pat_", "sk" + "-"]
markers += ["tok" + "en", "sec" + "ret"]
if any(marker.lower() in json.dumps(data).lower() for marker in markers):
    errors.append("sensitive value")
if not data.get("decisions"): errors.append("authority manifest has no decisions")
if not data.get("closure_rules"): errors.append("authority manifest has no closure rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification authority check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification authority is stale: run scripts/verificationauthoritygen.sh generate" \
      "verification authority generated output ok"
    validate_authority ;;
  *) echo "usage: scripts/verificationauthoritygen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
