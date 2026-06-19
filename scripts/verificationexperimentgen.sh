#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo
mode="${1:-check}"
out="${2:-docs/generated/verification-experiment-loop.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-experiment-json))' |
    python3 -m json.tool
}

validate_experiments() {
  python3 - "$repo" "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
repo, path, evidence_path = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
statuses, errors, seen, checks = {"proposed", "running", "checked", "promoted"}, [], set(), []
for row in data.get("experiments", []):
    if row["id"] in seen:
        errors.append(f"duplicate experiment {row['id']}")
    seen.add(row["id"])
    for key in ["plan", "do", "act"]:
        if not (repo / row[key]).exists():
            errors.append(f"{row['id']} missing {key}")
    command = row["check"]
    script = command.split()[0]
    if not command.startswith("scripts/") or not command.endswith(" check"):
        errors.append(f"{row['id']} bad check command")
    elif not (repo / script).exists():
        errors.append(f"{row['id']} missing check script")
    if row["status"] not in statuses:
        errors.append(f"{row['id']} bad status")
    if row.get("promoted") and row["status"] not in {"checked", "promoted"}:
        errors.append(f"{row['id']} cannot promote before check")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
    if command not in checks:
        checks.append(command)
if not data.get("experiments"):
    errors.append("experiment loop has no experiments")
if not data.get("closure_rules"):
    errors.append("experiment loop has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("\n".join(checks))
PY
}

check_experiments() {
  while IFS= read -r check; do
    [ -z "$check" ] && continue
    bash -lc "$check" >/dev/null
  done
  echo "verification experiment loop check ok"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated experiment loop is stale: run scripts/verificationexperimentgen.sh generate" \
      "verification experiment loop generated output ok"
    validate_experiments | check_experiments ;;
  *) echo "usage: scripts/verificationexperimentgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
