#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-genesis-charter.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-genesis-json))' |
    python3 -m json.tool
}

validate_genesis() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
charter, errors = data.get("charter", {}), []
if charter.get("review_owner", "").startswith("agent:"):
    errors.append("genesis owner cannot be an agent")
for field in ("bounded_contexts", "actors", "authority_rules", "non_goals"):
    if not charter.get(field): errors.append(f"missing {field}")
if not charter.get("revalidation"):
    errors.append("missing revalidation")
for artifact in charter.get("artifacts", []):
    if not (pathlib.Path.cwd() / artifact).exists():
        errors.append(f"missing artifact {artifact}")
for item in charter.get("evidence", []):
    if item not in outputs:
        errors.append(f"unknown evidence {item}")
if not data.get("closure_rules"):
    errors.append("genesis charter has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification genesis charter check ok")
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
      "generated verification genesis charter is stale: run scripts/verificationgenesisgen.sh generate" \
      "verification genesis charter generated output ok"
    validate_genesis
    ;;
  *) echo "usage: scripts/verificationgenesisgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
