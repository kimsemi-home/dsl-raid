#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-translation-verifier.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-translation-json))' |
    python3 -m json.tool
}

validate_translation() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" \
    "$repo/docs/generated/verification-context-map.json" \
    "$repo/docs/generated/verification-loss-ledger.json" <<'PY'
import json, sys
data, evidence, context, ledger = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
translations = {row["id"]: row for row in context["translations"]}
losses = {row["id"]: row for row in ledger["ledger"]}
seen, errors = set(), []
for row in data.get("checks", []):
    seen.add(row["translation"])
    source = translations.get(row["translation"])
    if not source: errors.append(f"{row['id']} unknown translation")
    elif source["loss_policy"] != row["loss_policy"]:
        errors.append(f"{row['id']} loss mismatch")
    loss = losses.get(row["loss_policy"])
    if not loss: errors.append(f"{row['id']} unknown loss policy")
    elif loss["loss_level"] != row["loss_level"]:
        errors.append(f"{row['id']} loss level mismatch")
    if row["loss_level"] == "L4": errors.append(f"{row['id']} forbidden L4 loss")
    if row["verdict"] != "verified": errors.append(f"{row['id']} not verified")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"{row['id']} unknown evidence {item}")
missing = set(translations) - seen
if missing: errors.append(f"missing translation checks {sorted(missing)}")
if not data.get("checks"): errors.append("translation verifier has no checks")
if not data.get("closure_rules"): errors.append("translation verifier has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification translation verifier check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated translation verifier is stale: run scripts/verificationtranslationgen.sh generate" \
      "verification translation verifier generated output ok"
    validate_translation ;;
  *) echo "usage: scripts/verificationtranslationgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
