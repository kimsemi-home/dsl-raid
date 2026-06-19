#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-reasoning-access.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-reasoning-access-json))' |
    python3 -m json.tool
}

validate_reasoning() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
tiers = {"R0", "R1", "R2", "R3", "R4"}
contexts = {"verification", "public-surface", "incident", "ontology", "release", "security-boundary"}
effects = {"transform-only", "analysis-only", "human-review", "governance-review", "blocked"}
ceilings, sensitive = {"low", "medium", "high"}, {"ontology", "release", "security-boundary"}
errors, seen = [], set()
for row in data.get("records", []):
    if row["id"] in seen:
        errors.append(f"duplicate reasoning record {row['id']}")
    seen.add(row["id"])
    if row["reasoning_tier"] not in tiers or row["context"] not in contexts:
        errors.append(f"{row['id']} bad tier or context")
    if row["authority_effect"] not in effects or row["confidence_ceiling"] not in ceilings:
        errors.append(f"{row['id']} bad effect or ceiling")
    if not row["abac"].startswith("abac:"):
        errors.append(f"{row['id']} missing domain ABAC")
    if row["owner"].startswith("agent:"):
        errors.append(f"{row['id']} owner cannot be an agent")
    if set(row["allowed"]) & set(row["blocked"]):
        errors.append(f"{row['id']} allowed and blocked overlap")
    if row["reasoning_tier"] == "R0" and row["authority_effect"] != "transform-only":
        errors.append(f"{row['id']} R0 must be transform only")
    if row["context"] in sensitive and row["authority_effect"] != "governance-review":
        errors.append(f"{row['id']} sensitive context must use governance review")
    if row["confidence_ceiling"] == "high" and row["reasoning_tier"] != "R4":
        errors.append(f"{row['id']} high confidence ceiling requires R4")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("records"):
    errors.append("reasoning manifest has no records")
if not data.get("closure_rules"):
    errors.append("reasoning manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification reasoning access check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated reasoning access is stale: run scripts/verificationreasoninggen.sh generate" \
      "verification reasoning access generated output ok"
    validate_reasoning ;;
  *) echo "usage: scripts/verificationreasoninggen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
