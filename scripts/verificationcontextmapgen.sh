#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-context-map.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-context-map-json))' |
    python3 -m json.tool
}

validate_context_map() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" "$repo/docs/generated/verification-loss-ledger.json" <<'PY'
import json, re, sys
data, evidence, ledger = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
losses = {row["id"] for row in ledger["ledger"]}
semver = re.compile(r"^\d+\.\d+\.\d+$")
errors = []
for row in data.get("translations", []):
    if row["source_context"] == row["target_context"]:
        errors.append(f"{row['id']} has identical contexts")
    for field in ("source_context", "target_context"):
        if not row[field].startswith("context:"):
            errors.append(f"{row['id']} bad {field}")
    for field in ("source_version", "target_version"):
        if not semver.match(row[field]):
            errors.append(f"{row['id']} bad {field}")
    if row["loss_policy"] not in losses:
        errors.append(f"{row['id']} missing loss policy {row['loss_policy']}")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("translations"):
    errors.append("context map has no translations")
if not data.get("closure_rules"):
    errors.append("context map has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification context map check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification context map is stale: run scripts/verificationcontextmapgen.sh generate" \
      "verification context map generated output ok"
    validate_context_map ;;
  *) echo "usage: scripts/verificationcontextmapgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
