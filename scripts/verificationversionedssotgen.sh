#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-versioned-ssot.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-versioned-ssot-json))' |
    python3 -m json.tool
}

validate_versioned_ssot() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, re, sys
path, evidence_path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs, semver = {row["output"] for row in evidence["generated_backends"]}, re.compile(r"^\d+\.\d+\.\d+$")
errors, seen = [], set()
for row in data.get("scopes", []):
    if row["id"] in seen: errors.append(f"duplicate scope {row['id']}")
    seen.add(row["id"])
    if not row["context"].startswith("context:"): errors.append(f"bad context {row['id']}")
    if not semver.match(row["ontology_version"]): errors.append(f"bad ontology version {row['id']}")
    if not semver.match(row["contract_version"]): errors.append(f"bad contract version {row['id']}")
    if not (pathlib.Path(sys.argv[1]).parents[2] / row["ssot"]).exists():
        errors.append(f"missing ssot {row['id']} {row['ssot']}")
    if row["authority"].startswith("agent:"): errors.append(f"agent authority {row['id']}")
    if not row.get("evidence"): errors.append(f"missing evidence {row['id']}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"unknown evidence {row['id']} {item}")
if not data.get("scopes"): errors.append("versioned ssot has no scopes")
if not data.get("closure_rules"): errors.append("versioned ssot has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification versioned ssot check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification versioned ssot is stale: run scripts/verificationversionedssotgen.sh generate" \
      "verification versioned ssot generated output ok"
    validate_versioned_ssot ;;
  *) echo "usage: scripts/verificationversionedssotgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
