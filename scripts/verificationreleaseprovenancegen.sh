#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-release-provenance.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-release-provenance-json))' |
    python3 -m json.tool
}

validate_release_provenance() {
  python3 - "$repo/$out" <<'PY'
import json, os, sys
data = json.load(open(sys.argv[1]))
errors, seen = [], set()
for row in data.get("gates", []):
    if row["id"] in seen:
        errors.append(f"duplicate release gate {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "required":
        errors.append(f"{row['id']} must be required")
    if not os.path.exists(row["workflow"]):
        errors.append(f"{row['id']} missing workflow")
        continue
    text = open(row["workflow"]).read()
    for item in row.get("requires", []):
        if item not in text:
            errors.append(f"{row['id']} missing release requirement {item}")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
if len(data.get("gates", [])) < 5:
    errors.append("release provenance needs all promotion gates")
if not data.get("closure_rules"):
    errors.append("release provenance manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification release provenance check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated release provenance is stale: run scripts/verificationreleaseprovenancegen.sh generate" \
      "verification release provenance generated output ok"
    validate_release_provenance ;;
  *) echo "usage: scripts/verificationreleaseprovenancegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
