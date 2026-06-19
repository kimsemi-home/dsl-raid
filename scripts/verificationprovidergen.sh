#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-provider-compat.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-provider-compat-json))' |
    python3 -m json.tool
}

validate_provider() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("records", []):
    if row["id"] in seen:
        errors.append(f"duplicate provider compat {row['id']}")
    seen.add(row["id"])
    if not row["provider"].startswith("provider:"):
        errors.append(f"{row['id']} bad provider")
    if not row["runtime"].startswith("runtime:"):
        errors.append(f"{row['id']} bad runtime")
    if not row["protocol"].startswith("protocol:"):
        errors.append(f"{row['id']} bad protocol")
    missing = sorted(set(row.get("requires", [])) - set(row.get("supports", [])))
    if missing and row["status"] != "blocked":
        errors.append(f"{row['id']} missing capability without block: {missing}")
    if not missing and row["status"] != "compatible":
        errors.append(f"{row['id']} compatible provider marked {row['status']}")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("records"):
    errors.append("provider compatibility manifest has no records")
if not data.get("closure_rules"):
    errors.append("provider compatibility manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification provider compatibility check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated provider compatibility is stale: run scripts/verificationprovidergen.sh generate" \
      "verification provider compatibility generated output ok"
    validate_provider ;;
  *) echo "usage: scripts/verificationprovidergen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
