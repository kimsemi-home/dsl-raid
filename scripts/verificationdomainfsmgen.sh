#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-domain-fsm.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-domain-fsm-json))' |
    python3 -m json.tool
}

validate_domain_fsm() {
  python3 - "$repo/$out" "$repo" <<'PY'
import json, os, subprocess, sys
data, repo = json.load(open(sys.argv[1])), sys.argv[2]
errors, seen = [], set()
def exists(item):
    return item.startswith("stdout:") or os.path.exists(os.path.join(repo, item))
def run(command):
    return subprocess.run(["bash", "-lc", command], cwd=repo, text=True, capture_output=True)
for row in data.get("surfaces", []):
    if row["id"] in seen: errors.append(f"duplicate surface {row['id']}")
    seen.add(row["id"])
    if not exists(row["source"]): errors.append(f"{row['id']} missing source")
    for item in row.get("generated", []):
        if not exists(item): errors.append(f"{row['id']} missing generated {item}")
    for item in row.get("evidence", []):
        if not exists(item): errors.append(f"{row['id']} missing evidence {item}")
    result = run(row["command"])
    expected = row["assertion"].removeprefix("stdout:")
    if result.returncode or expected not in result.stdout:
        errors.append(f"{row['id']} expected stdout {expected!r}")
    if row["kind"] == "human-diagram" and not row.get("lossy"):
        errors.append(f"{row['id']} diagram must be lossy")
kinds = {row["kind"] for row in data.get("surfaces", [])}
for required in ("lisp-dsl", "canonical-ir", "runtime-code", "human-diagram"):
    if required not in kinds: errors.append(f"missing {required} surface")
if not data.get("closure_rules"): errors.append("domain fsm has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification domain fsm check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated domain fsm is stale: run scripts/verificationdomainfsmgen.sh generate" \
      "verification domain fsm generated output ok"
    validate_domain_fsm ;;
  *) echo "usage: scripts/verificationdomainfsmgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
