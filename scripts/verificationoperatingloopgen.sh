#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-operating-loop.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-operating-loop-json))' |
    python3 -m json.tool
}

validate_operating_loop() {
  python3 - "$repo/$out" "$repo" <<'PY'
import json, os, subprocess, sys
data, repo = json.load(open(sys.argv[1])), sys.argv[2]
errors, seen, orders, phases = [], set(), [], []
def exists(path): return os.path.exists(os.path.join(repo, path))
def run(command):
    return subprocess.run(["bash", "-lc", command], cwd=repo, text=True, capture_output=True)
for row in data.get("stages", []):
    if row["id"] in seen: errors.append(f"duplicate stage {row['id']}")
    seen.add(row["id"]); orders.append(row["order"]); phases.append(row["phase"])
    for key in ("input", "output"):
        if not exists(row[key]): errors.append(f"{row['id']} missing {key} {row[key]}")
    for item in row.get("evidence", []):
        if not exists(item): errors.append(f"{row['id']} missing evidence {item}")
    result = run(row["command"])
    expected = row["assertion"].removeprefix("stdout:")
    if result.returncode or expected not in result.stdout:
        errors.append(f"{row['id']} expected stdout {expected!r}")
required = ["observation","evidence-collection","evidence-quality-check","ontology-mapping",
            "version-attribution","root-cause-discovery","hypothesis","target-verification",
            "change-candidate","external-confidence-assessment","authority-gate","change",
            "verification-evidence","conformance-evidence","evidence-graph-update",
            "retrospective","knowledge-update"]
if phases != required: errors.append("operating loop phases are not canonical")
if orders != list(range(1, len(orders) + 1)): errors.append("operating loop order is not contiguous")
if not data.get("closure_rules"): errors.append("operating loop has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification operating loop check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated operating loop is stale: run scripts/verificationoperatingloopgen.sh generate" \
      "verification operating loop generated output ok"
    validate_operating_loop ;;
  *) echo "usage: scripts/verificationoperatingloopgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
