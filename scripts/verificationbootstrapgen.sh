#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo
mode="${1:-check}"
out="${2:-docs/generated/verification-bootstrap-sequence.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-bootstrap-json))' |
    python3 -m json.tool
}

validate_bootstrap() {
  python3 - "$repo" "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
repo, path, evidence_path = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen, checks, orders = [], set(), [], []
for row in data.get("stages", []):
    if row["id"] in seen:
        errors.append(f"duplicate bootstrap stage {row['id']}")
    seen.add(row["id"])
    orders.append(row["order"])
    if row.get("status") != "checked":
        errors.append(f"{row['id']} must be checked")
    for key in ["input", "output"]:
        if not (repo / row[key]).exists():
            errors.append(f"{row['id']} missing {key}")
    command = row["check"]
    script = command.split()[0]
    if not command.startswith("scripts/") or not command.endswith(" check"):
        errors.append(f"{row['id']} bad check command")
    elif not (repo / script).exists():
        errors.append(f"{row['id']} missing check script")
    if command not in checks:
        checks.append(command)
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if orders != sorted(orders):
    errors.append("bootstrap stages are not ordered")
if not data.get("stages"):
    errors.append("bootstrap sequence has no stages")
if not data.get("closure_rules"):
    errors.append("bootstrap sequence has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("\n".join(checks))
PY
}

check_bootstrap_stages() {
  while IFS= read -r check; do
    [ -z "$check" ] && continue
    bash -lc "$check" >/dev/null
  done
  echo "verification bootstrap sequence check ok"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated bootstrap sequence is stale: run scripts/verificationbootstrapgen.sh generate" \
      "verification bootstrap sequence generated output ok"
    validate_bootstrap | check_bootstrap_stages ;;
  *) echo "usage: scripts/verificationbootstrapgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
