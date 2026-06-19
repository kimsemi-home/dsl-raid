#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-loss-ledger.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-loss-json))' |
    python3 -m json.tool
}

validate_ledger() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json
import sys

ledger, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
errors = []

if not ledger.get("ledger"):
    errors.append("loss ledger has no entries")

for row in ledger.get("ledger", []):
    if row.get("loss_level") == "L4":
        errors.append(f"{row['id']} records forbidden L4 loss")
    if row.get("evidence") not in outputs:
        errors.append(f"{row['id']} evidence is not generated output")
    if row.get("target") not in outputs:
        errors.append(f"{row['id']} target is not generated output")

if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)

print("verification loss ledger semantic check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out"
    ;;
  check)
    dslraid_generated_check \
      "$out" \
      "generated verification loss ledger is stale: run scripts/verificationlossgen.sh generate" \
      "verification loss ledger generated output ok"
    validate_ledger
    ;;
  *)
    echo "usage: scripts/verificationlossgen.sh [generate|check] [out]" >&2
    exit 2
    ;;
esac
