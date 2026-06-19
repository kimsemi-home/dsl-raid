#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-adversarial-review.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-adversarial-review-json))' |
    python3 -m json.tool
}

validate_adversarial() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
scopes = {"release", "conformance", "authority", "experiment"}
effects = {"review-required", "human-review", "authority-blocked"}
errors, seen = [], set()
for row in data.get("probes", []):
    if row["id"] in seen:
        errors.append(f"duplicate adversarial probe {row['id']}")
    seen.add(row["id"])
    if row["scope"] not in scopes or row["authority_effect"] not in effects:
        errors.append(f"{row['id']} bad scope or effect")
    if not row["reviewer"].startswith("reviewer:"):
        errors.append(f"{row['id']} reviewer must be reviewer")
    if row["owner"].startswith("agent:"):
        errors.append(f"{row['id']} owner cannot be agent")
    if row["severity"] in {"D3", "D4"} and row["authority_effect"] != "human-review":
        errors.append(f"{row['id']} severe probe must require human review")
    if not row.get("detects"):
        errors.append(f"{row['id']} names no failure mode")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("probes"):
    errors.append("adversarial manifest has no probes")
if not data.get("closure_rules"):
    errors.append("adversarial manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification adversarial review check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated adversarial review is stale: run scripts/verificationadversarialgen.sh generate" \
      "verification adversarial review generated output ok"
    validate_adversarial ;;
  *) echo "usage: scripts/verificationadversarialgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
