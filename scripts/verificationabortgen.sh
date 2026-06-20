#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-abort-evidence.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-abort-evidence-json))' |
    python3 -m json.tool
}

validate_abort_evidence() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
errors, seen = [], set()
for row in data.get("bundles", []):
    if row["id"] in seen: errors.append(f"duplicate abort bundle {row['id']}")
    seen.add(row["id"])
    if not row["lease"].startswith("lease:"): errors.append(f"bad lease {row['id']}")
    if row["decision"] != "decision:abort": errors.append(f"bad decision {row['id']}")
    if not row["bundle"].startswith("bundle:"): errors.append(f"bad bundle {row['id']}")
    if row["authority_effect"] != "commit-blocked":
        errors.append(f"abort must block commit {row['id']}")
    if row["evidence_update"] not in outputs:
        errors.append(f"unknown evidence update {row['id']}")
    if not row.get("classified_artifacts"):
        errors.append(f"no classified artifacts {row['id']}")
    if not row.get("recheck_claims"):
        errors.append(f"no recheck claims {row['id']}")
    for claim in row.get("recheck_claims", []):
        if not claim.startswith("claim:"): errors.append(f"bad claim {row['id']} {claim}")
    for item in row.get("evidence", []):
        if item not in outputs and not pathlib.Path(item).exists():
            errors.append(f"unknown evidence {row['id']} {item}")
if not data.get("bundles"): errors.append("abort evidence has no bundles")
if not data.get("closure_rules"): errors.append("abort evidence has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification abort evidence check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated abort evidence is stale: run scripts/verificationabortgen.sh generate" \
      "verification abort evidence generated output ok"
    validate_abort_evidence ;;
  *) echo "usage: scripts/verificationabortgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
