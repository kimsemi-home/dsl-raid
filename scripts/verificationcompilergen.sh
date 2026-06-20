#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-governed-compiler.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-governed-compiler-json))' |
    python3 -m json.tool
}

validate_compiler_farm() {
  python3 - "$repo/$out" "$repo" <<'PY'
import json, os, subprocess, sys
data, repo = json.load(open(sys.argv[1])), sys.argv[2]
errors, seen, stages, trusts, orders = [], set(), [], [], []
required = ["spec", "candidate", "validation", "evidence", "external-confidence", "authority"]
def exists(path): return os.path.exists(os.path.join(repo, path))
def run(command):
    return subprocess.run(["bash", "-lc", command], cwd=repo, text=True, capture_output=True)
for row in data.get("stages", []):
    if row["id"] in seen: errors.append(f"duplicate stage {row['id']}")
    seen.add(row["id"]); stages.append(row["stage"]); trusts.append(row["trust"])
    orders.append(row["order"])
    for key in ("input", "output"):
        if not exists(row[key]): errors.append(f"{row['id']} missing {key} {row[key]}")
    for item in row.get("evidence", []):
        if not exists(item): errors.append(f"{row['id']} missing evidence {item}")
    result = run(row["command"])
    expected = row["assertion"].removeprefix("stdout:")
    if result.returncode or expected not in result.stdout:
        errors.append(f"{row['id']} expected stdout {expected!r}")
if stages != required: errors.append("governed compiler stages are not canonical")
if orders != list(range(1, len(orders) + 1)): errors.append("stage order is not contiguous")
if trusts[1:2] != ["candidate"]: errors.append("agent output must remain candidate trust")
if trusts[-1:] != ["gated"]: errors.append("authority must be the final gated stage")
if not data.get("closure_rules"): errors.append("governed compiler has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification governed compiler check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated governed compiler is stale: run scripts/verificationcompilergen.sh generate" \
      "verification governed compiler generated output ok"
    validate_compiler_farm ;;
  *) echo "usage: scripts/verificationcompilergen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
