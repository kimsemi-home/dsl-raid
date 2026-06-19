#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-source-shape.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-source-shape-json))' |
    python3 -m json.tool
}

validate_source_shape() {
  python3 - "$repo/$out" <<'PY'
import json, os, subprocess, sys
data = json.load(open(sys.argv[1]))
errors, seen = [], set()
required = {"line-budget", "surface-boundary", "ssot-boundary", "generated-ownership"}
kinds = {row.get("kind") for row in data.get("budgets", [])}
for row in data.get("budgets", []):
    if row["id"] in seen:
        errors.append(f"duplicate source shape budget {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "required":
        errors.append(f"{row['id']} must be required")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
line = next((r for r in data.get("budgets", []) if r["kind"] == "line-budget"), {})
if line.get("limit") != "75" or "check-source-lines.sh" not in line.get("command", ""):
    errors.append("line budget must require scripts/check-source-lines.sh at limit 75")
if required - kinds:
    errors.append(f"missing source shape kinds {sorted(required - kinds)}")
if "scripts/check-source-lines.sh" not in open(".github/workflows/ci.yml").read():
    errors.append("CI must enforce source line budget")
if "scripts/verificationsourcegen.sh" not in open("crates/dslraid-cli/src/commands/quality/lisp/scripts.rs").read():
    errors.append("quality command must run source shape verification")
if not data.get("closure_rules"):
    errors.append("source shape manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
subprocess.run(["bash", "scripts/check-source-lines.sh"], check=True)
print("verification source shape check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated source shape is stale: run scripts/verificationsourcegen.sh generate" \
      "verification source shape generated output ok"
    validate_source_shape ;;
  *) echo "usage: scripts/verificationsourcegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
