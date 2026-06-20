#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-knowledge-conversion.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-knowledge-conversion-json))' |
    python3 -m json.tool
}

validate_knowledge_conversion() {
  python3 - "$repo/$out" "$repo" <<'PY'
import json, os, subprocess, sys
data, repo = json.load(open(sys.argv[1])), sys.argv[2]
errors, seen, orders, phases = [], set(), [], []
def exists(path): return os.path.exists(os.path.join(repo, path))
def run(command):
    return subprocess.run(["bash", "-lc", command], cwd=repo, text=True, capture_output=True)
for row in data.get("steps", []):
    if row["id"] in seen: errors.append(f"duplicate step {row['id']}")
    seen.add(row["id"]); orders.append(row["order"]); phases.append(row["phase"])
    for key in ("input", "output"):
        if not exists(row[key]): errors.append(f"{row['id']} missing {key} {row[key]}")
    for item in row.get("evidence", []):
        if not exists(item): errors.append(f"{row['id']} missing evidence {item}")
    result = run(row["command"])
    expected = row["assertion"].removeprefix("stdout:")
    if result.returncode or expected not in result.stdout:
        errors.append(f"{row['id']} expected stdout {expected!r}")
required = ["incompleteness-visible","failure-signaled","evidence-captured",
            "root-cause-mapped","debt-recorded","incident-learned",
            "knowledge-updated","version-propagated","runtime-revalidated"]
if phases != required: errors.append("knowledge conversion phases are not canonical")
if orders != list(range(1, len(orders) + 1)): errors.append("knowledge conversion order is not contiguous")
if not data.get("closure_rules"): errors.append("knowledge conversion has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification knowledge conversion check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated knowledge conversion is stale: run scripts/verificationknowledgegen.sh generate" \
      "verification knowledge conversion generated output ok"
    validate_knowledge_conversion ;;
  *) echo "usage: scripts/verificationknowledgegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
