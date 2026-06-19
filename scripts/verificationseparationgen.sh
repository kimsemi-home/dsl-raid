#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-evidence-separation.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-evidence-separation-json))' |
    python3 -m json.tool
}

validate_separation() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
root = pathlib.Path.cwd()
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("records", []):
    if row["id"] in seen:
        errors.append(f"duplicate separation record {row['id']}")
    seen.add(row["id"])
    if not row["interpretation"].startswith("interpretation:"):
        errors.append(f"{row['id']} interpretation must be explicit")
    if not row["claim"].startswith("claim:"):
        errors.append(f"{row['id']} claim must be explicit")
    if row["raw_evidence"] == row["artifact"]:
        errors.append(f"{row['id']} raw evidence cannot equal artifact")
    if row["policy"] != "raw-to-claim-via-interpretation":
        errors.append(f"{row['id']} unsupported policy")
    for field in ("raw_evidence", "artifact"):
        if not (root / row[field]).exists():
            errors.append(f"{row['id']} missing {field} {row[field]}")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown generated evidence {item}")
if not data.get("records"):
    errors.append("evidence separation manifest has no records")
if not data.get("closure_rules"):
    errors.append("evidence separation manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification evidence separation check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out"
    ;;
  check)
    dslraid_generated_check "$out" \
      "generated evidence separation is stale: run scripts/verificationseparationgen.sh generate" \
      "verification evidence separation generated output ok"
    validate_separation
    ;;
  *) echo "usage: scripts/verificationseparationgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
