#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-evidence-graph.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-evidence-graph-json))' |
    python3 -m json.tool
}

validate_graph() {
  python3 - "$repo/$out" <<'PY'
import json, os, sys
data = json.load(open(sys.argv[1]))
errors, nodes, edges = [], set(), set()
required = {"observes", "interprets", "supports", "gates", "updates"}
for row in data.get("nodes", []):
    if row["id"] in nodes:
        errors.append(f"duplicate evidence graph node {row['id']}")
    nodes.add(row["id"])
    if not os.path.exists(row.get("artifact", "")):
        errors.append(f"{row['id']} missing artifact {row.get('artifact')}")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
relations = set()
for row in data.get("edges", []):
    if row["id"] in edges:
        errors.append(f"duplicate evidence graph edge {row['id']}")
    edges.add(row["id"])
    if row.get("from") not in nodes or row.get("to") not in nodes:
        errors.append(f"{row['id']} references an unknown endpoint")
    if row.get("from") == row.get("to"):
        errors.append(f"{row['id']} must not be a self loop")
    if row.get("status") != "linked":
        errors.append(f"{row['id']} must be linked")
    relations.add(row.get("relation"))
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
if required - relations:
    errors.append(f"missing evidence graph relations {sorted(required - relations)}")
if not any(e.get("relation") == "updates" for e in data.get("edges", [])):
    errors.append("evidence graph must close through feedback")
if not data.get("closure_rules"):
    errors.append("evidence graph manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification evidence graph check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated evidence graph is stale: run scripts/verificationevidencegraphgen.sh generate" \
      "verification evidence graph generated output ok"
    validate_graph ;;
  *) echo "usage: scripts/verificationevidencegraphgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
