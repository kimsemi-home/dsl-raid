#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-quality-closure.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-quality-closure-json))' |
    python3 -m json.tool
}

validate_quality_closure() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" "$repo" <<'PY'
import json, pathlib, sys
data = json.load(open(sys.argv[1]))
evidence = json.load(open(sys.argv[2]))
repo = pathlib.Path(sys.argv[3])
quality_root = repo / "crates/dslraid-cli/src/commands/quality"
quality_text = "\n".join(path.read_text() for path in quality_root.rglob("*.rs"))
home_prefix = "/" + "Users" + "/"
raw = pathlib.Path(sys.argv[1]).read_text()
errors = []
if home_prefix in raw:
    errors.append("quality closure leaked a private local path")
backend_rows = {
    (row["backend"], row["output"], row["generator"])
    for row in evidence.get("generated_backends", [])
}
closure_rows = {
    (row["backend"], row["output"], row["generator"])
    for row in data.get("enforced_generators", [])
}
if backend_rows != closure_rows:
    errors.append("quality closure rows differ from generated backend evidence")
for backend, _output, generator in sorted(closure_rows):
    if not (repo / generator).exists():
        errors.append(f"{backend} missing generator {generator}")
    if generator not in quality_text:
        errors.append(f"{backend} generator is not enforced by quality: {generator}")
    if not generator.startswith("scripts/"):
        errors.append(f"{backend} generator must be a script path")
if not data.get("enforced_generators"):
    errors.append("quality closure has no enforced generators")
if not data.get("closure_rules"):
    errors.append("quality closure has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification quality closure check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification quality closure is stale: run scripts/verificationqualitygen.sh generate" \
      "verification quality closure generated output ok"
    validate_quality_closure ;;
  *) echo "usage: scripts/verificationqualitygen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
