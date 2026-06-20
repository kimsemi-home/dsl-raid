#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-workflow-lineage.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-workflow-lineage-json))' |
    python3 -m json.tool
}

validate_lineage() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, os, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
nodes = {row["id"] for row in evidence["verification_nodes"]}
outputs = {row["output"]: row for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("lineages", []):
    rid, artifact = row["id"], row["artifact"]
    if rid in seen: errors.append(f"duplicate lineage {rid}")
    seen.add(rid)
    if not set(row["graph_nodes"]).issubset(nodes):
        errors.append(f"{rid} references unknown graph node")
    if not os.path.exists(artifact):
        errors.append(f"{rid} missing artifact {artifact}")
    if not os.path.exists(row["generator"]):
        errors.append(f"{rid} missing generator {row['generator']}")
    if not row["check"].endswith(" check"):
        errors.append(f"{rid} check must be a check command")
    if artifact in outputs and outputs[artifact]["generator"] != row["generator"]:
        errors.append(f"{rid} generator mismatch")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{rid} missing evidence {item}")
if {row["surface"] for row in data.get("lineages", [])} != {
    "github-actions", "gitlab-ci", "local-makefile", "bazel", "release-check-provider"
}:
    errors.append("workflow lineage surface set mismatch")
if not data.get("closure_rules"): errors.append("workflow lineage has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification workflow lineage check ok")
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
      "generated workflow lineage is stale: run scripts/verificationlineagegen.sh generate" \
      "verification workflow lineage generated output ok"
    validate_lineage
    ;;
  *) echo "usage: scripts/verificationlineagegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
