#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-semantic-os.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-semantic-os-json))' |
    python3 -m json.tool
}

validate_semantic_os() {
  python3 - "$repo/$out" "$repo" <<'PY'
import json, os, subprocess, sys
data, repo = json.load(open(sys.argv[1])), sys.argv[2]
errors, roles, seen = [], set(), set()
def exists(path):
    return path.startswith("stdout:") or os.path.exists(os.path.join(repo, path))
def run(command):
    return subprocess.run(["bash", "-lc", command], cwd=repo, text=True, capture_output=True)
for row in data.get("layers", []):
    if row["id"] in seen:
        errors.append(f"duplicate layer {row['id']}")
    seen.add(row["id"]); roles.add(row["role"])
    for key in ("source", "artifact"):
        if not exists(row[key]):
            errors.append(f"{row['id']} missing {key} {row[key]}")
    for item in row.get("evidence", []):
        if not exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
    result = run(row["command"])
    expected = row["assertion"].removeprefix("stdout:")
    if result.returncode or expected not in result.stdout:
        errors.append(f"{row['id']} expected stdout {expected!r}")
required = {"firmware","kernel","filesystem","userland","driver","log","scheduler","court"}
missing = sorted(required - roles)
if missing:
    errors.append(f"missing semantic os roles {missing}")
if not data.get("closure_rules"):
    errors.append("semantic os has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification semantic os check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated semantic os is stale: run scripts/verificationsemanticosgen.sh generate" \
      "verification semantic os generated output ok"
    validate_semantic_os ;;
  *) echo "usage: scripts/verificationsemanticosgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
