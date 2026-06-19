#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-runtime-trace.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-runtime-trace-json))' |
    python3 -m json.tool
}

validate_runtime_trace() {
  python3 - "$repo" "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
repo, path, evidence_path = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
statuses, errors, seen, triples = {"covered", "uncovered", "failed"}, [], set(), set()
for row in data.get("mappings", []):
    if row["id"] in seen: errors.append(f"duplicate runtime trace {row['id']}")
    seen.add(row["id"])
    for key in ["design_ir", "trace", "coverage"]:
        if not (repo / row[key]).exists(): errors.append(f"{row['id']} missing {key}")
    if row["runtime_subject"] != row["design_subject"]:
        errors.append(f"{row['id']} runtime subject differs from design subject")
    if row["coverage_status"] not in statuses:
        errors.append(f"{row['id']} bad coverage status")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"{row['id']} unknown evidence {item}")
    triples.add((row["design_ir"], row["trace"], row["coverage"]))
if not data.get("mappings"): errors.append("runtime trace manifest has no mappings")
if not data.get("closure_rules"): errors.append("runtime trace manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
for triple in sorted(triples):
    print("\t".join(triple))
PY
}

check_runtime_commands() {
  while IFS=$'\t' read -r design trace coverage; do
    cargo run -p dslraid-cli -- trace check "$trace" --design-ir "$design" >/dev/null
    cargo run -p dslraid-cli -- coverage check "$coverage" --design-ir "$design" >/dev/null
  done
  echo "verification runtime trace check ok"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated runtime trace map is stale: run scripts/verificationruntimegen.sh generate" \
      "verification runtime trace generated output ok"
    validate_runtime_trace | check_runtime_commands ;;
  *) echo "usage: scripts/verificationruntimegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
