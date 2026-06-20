#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"
dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-executable-knowledge.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-executable-knowledge-json))' |
    python3 -m json.tool
}

validate_executable_knowledge() {
  python3 - "$repo/$out" "$repo" <<'PY'
import json, os, subprocess, sys
data, repo = json.load(open(sys.argv[1])), sys.argv[2]
errors, seen, kinds = [], set(), []
required = ["dsl", "specification", "ontology", "policy", "schema", "contract",
            "manifest", "ir", "verification-rule", "migration-rule",
            "translation-manifest", "evidence-policy"]
def exists(path): return os.path.exists(os.path.join(repo, path))
def run(command):
    return subprocess.run(["bash", "-lc", command], cwd=repo, text=True, capture_output=True)
for row in data.get("records", []):
    if row["id"] in seen: errors.append(f"duplicate record {row['id']}")
    seen.add(row["id"]); kinds.append(row["kind"])
    if not exists(row["source"]): errors.append(f"{row['id']} missing source")
    for key in ("generated", "evidence"):
        for item in row.get(key, []):
            if not exists(item): errors.append(f"{row['id']} missing {key} {item}")
    result = run(row["command"])
    expected = row["assertion"].removeprefix("stdout:")
    if result.returncode or expected not in result.stdout:
        errors.append(f"{row['id']} expected stdout {expected!r}")
    if row["source"] in row.get("generated", []):
        errors.append(f"{row['id']} source cannot equal generated artifact")
if kinds != required: errors.append("executable knowledge kinds are not canonical")
if not data.get("closure_rules"): errors.append("executable knowledge has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification executable knowledge check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated executable knowledge is stale: run scripts/verificationexecutablegen.sh generate" \
      "verification executable knowledge generated output ok"
    validate_executable_knowledge ;;
  *) echo "usage: scripts/verificationexecutablegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
