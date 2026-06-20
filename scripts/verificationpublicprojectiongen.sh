#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-public-projection.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-public-projection-json))' |
    python3 -m json.tool
}

validate_public_projection() {
  python3 - "$repo/$out" <<'PY'
import json, os, sys
data = json.load(open(sys.argv[1]))
errors, seen = [], set()
required = {"include", "exclude", "redact"}
for row in data.get("decisions", []):
    if row["id"] in seen:
        errors.append(f"duplicate decision {row['id']}")
    seen.add(row["id"])
    if row.get("kind") not in {"include", "exclude", "redact"}:
        errors.append(f"bad kind {row['id']}")
    if row.get("effect") not in {"included", "excluded", "blocked"}:
        errors.append(f"bad effect {row['id']}")
    if row.get("source") in {"private-data", "secret-bearing-artifact"}:
        if row.get("effect") == "included":
            errors.append(f"private source included by {row['id']}")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
kinds = {row.get("kind") for row in data.get("decisions", [])}
if required - kinds:
    errors.append(f"missing decision kinds {sorted(required - kinds)}")
if not data.get("closure_rules"):
    errors.append("public projection manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification public projection check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated public projection is stale: run scripts/verificationpublicprojectiongen.sh generate" \
      "verification public projection generated output ok"
    validate_public_projection
    bash scripts/privacycheck.sh check ;;
  *) echo "usage: scripts/verificationpublicprojectiongen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
