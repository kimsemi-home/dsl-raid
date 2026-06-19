#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-run-manifest.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-run-manifest-json))' |
    python3 -m json.tool
}

validate_run_manifest() {
  python3 - "$repo" "$repo/$out" "$repo/docs/generated/verification-evidence.json" <<'PY'
import json, pathlib, sys
repo, path, evidence_path = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]
data, evidence = json.load(open(path)), json.load(open(evidence_path))
outputs = {row["output"] for row in evidence["generated_backends"]}
statuses = {"proposed", "running", "verified", "rejected", "quarantined", "aborted"}
errors, seen, triples = [], set(), set()
for row in data.get("records", []):
    if row["id"] in seen:
        errors.append(f"duplicate run manifest {row['id']}")
    seen.add(row["id"])
    if not row["run"].startswith("agent-run:"):
        errors.append(f"{row['id']} bad run id")
    if row["status"] not in statuses:
        errors.append(f"{row['id']} bad status")
    if not row["authority"].startswith(("gate:", "sidecar:", "steward:", "authority:")):
        errors.append(f"{row['id']} bad authority")
    for key in ["manifest", "schema", "generated_doc"]:
        if not (repo / row[key]).exists():
            errors.append(f"{row['id']} missing {key}")
    for item in row.get("evidence", []):
        if item not in outputs:
            errors.append(f"{row['id']} unknown evidence {item}")
    triples.add((row["schema"], row["manifest"], row["generated_doc"]))
if not data.get("records"):
    errors.append("run manifest verification has no records")
if not data.get("closure_rules"):
    errors.append("run manifest verification has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
for triple in sorted(triples):
    print("\t".join(triple))
PY
}

check_run_records() {
  while IFS=$'\t' read -r schema manifest doc; do
    cargo run -p dslraid-cli -- schema validate "$schema" "$manifest" >/dev/null
    bash scripts/agentmanifestgen.sh check "$manifest" "$doc" >/dev/null
  done
  echo "verification run manifest check ok"
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated run manifest verification is stale: run scripts/verificationrunmanifestgen.sh generate" \
      "verification run manifest generated output ok"
    validate_run_manifest | check_run_records ;;
  *) echo "usage: scripts/verificationrunmanifestgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
