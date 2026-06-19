#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-access-policy.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-access-policy-json))' |
    python3 -m json.tool
}

validate_access() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, sys
data, evidence = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
roles = {"producer", "verifier", "steward", "governance"}
contexts = {"public-surface", "private-data", "security-boundary", "release", "ontology"}
decisions, errors, seen = {"allow", "deny", "escalate"}, [], set()
for row in data.get("policies", []):
    if row["id"] in seen:
        errors.append(f"duplicate access policy {row['id']}")
    seen.add(row["id"])
    if row["role"] not in roles:
        errors.append(f"{row['id']} bad role")
    if row["context"] not in contexts:
        errors.append(f"{row['id']} bad context")
    if row["decision"] not in decisions:
        errors.append(f"{row['id']} bad decision")
    if row["context"] == "private-data" and row["decision"] == "allow":
        errors.append(f"{row['id']} cannot allow private data")
    if not row["rbac"].startswith("rbac:") or not row["abac"].startswith("abac:"):
        errors.append(f"{row['id']} missing rbac or abac")
    if row["authority"].startswith("agent:"):
        errors.append(f"{row['id']} authority cannot be an agent")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
if not data.get("policies"):
    errors.append("access policy manifest has no policies")
if not data.get("closure_rules"):
    errors.append("access policy manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification access policy check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated access policy is stale: run scripts/verificationaccessgen.sh generate" \
      "verification access policy generated output ok"
    validate_access ;;
  *) echo "usage: scripts/verificationaccessgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
