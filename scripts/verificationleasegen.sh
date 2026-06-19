#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-lease.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-lease-json))' |
    python3 -m json.tool
}

validate_lease() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("leases", []):
    if row["id"] in seen: errors.append(f"duplicate lease {row['id']}")
    seen.add(row["id"])
    if row["status"] not in {"issued", "active", "finished", "aborted", "escalated"}:
        errors.append(f"bad status {row['id']}")
    if row["status"] in {"aborted", "escalated"} and row["authority_effect"] != "authority-blocked":
        errors.append(f"authority not blocked {row['id']}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"unknown evidence {row['id']} {item}")
for row in data.get("abort_rules", []):
    if row.get("effect") != "authority-blocked": errors.append(f"bad abort effect {row['id']}")
if not data.get("leases"): errors.append("lease manifest has no leases")
if not data.get("abort_rules"): errors.append("lease manifest has no abort rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification lease check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification lease is stale: run scripts/verificationleasegen.sh generate" \
      "verification lease generated output ok"
    validate_lease ;;
  *) echo "usage: scripts/verificationleasegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
