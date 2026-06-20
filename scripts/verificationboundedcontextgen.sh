#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-bounded-context.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-bounded-context-json))' |
    python3 -m json.tool
}

validate_contexts() {
  python3 - "$repo/$out" "$repo/docs/generated/verification-evidence.json" \
    "$repo/docs/generated/verification-versioned-ssot.json" \
    "$repo/docs/generated/verification-context-map.json" <<'PY'
import json, re, sys
data, evidence, ssot, cmap = [json.load(open(path)) for path in sys.argv[1:]]
outputs = {row["output"] for row in evidence["generated_backends"]}
contexts = {row["id"]: row for row in data.get("contexts", [])}
semver, errors = re.compile(r"^\d+\.\d+\.\d+$"), []
for row in data.get("contexts", []):
    if not row["id"].startswith("context:"): errors.append(f"{row['id']} bad id")
    for field in ("ontology_version", "contract_version"):
        if not semver.match(row[field]): errors.append(f"{row['id']} bad {field}")
    if row["authority"].startswith("agent:"): errors.append(f"{row['id']} agent authority")
    if not row.get("terms"): errors.append(f"{row['id']} missing terms")
    for term in row.get("terms", []):
        if "@" not in term: errors.append(f"{row['id']} bare term {term}")
    for item in row.get("evidence", []):
        if item not in outputs: errors.append(f"{row['id']} unknown evidence {item}")
refs = {row["context"] for row in ssot.get("scopes", [])}
for row in cmap.get("translations", []):
    refs.add(row["source_context"]); refs.add(row["target_context"])
missing = refs - set(contexts)
if missing: errors.append(f"missing bounded contexts {sorted(missing)}")
if len(contexts) != len(data.get("contexts", [])): errors.append("duplicate context id")
if not data.get("contexts"): errors.append("bounded context has no contexts")
if not data.get("closure_rules"): errors.append("bounded context has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification bounded context check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated bounded context is stale: run scripts/verificationboundedcontextgen.sh generate" \
      "verification bounded context generated output ok"
    validate_contexts ;;
  *) echo "usage: scripts/verificationboundedcontextgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
