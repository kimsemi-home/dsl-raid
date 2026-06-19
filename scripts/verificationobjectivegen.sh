#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-objective-coverage.json}"

generate() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-objective-coverage-json))' |
    python3 -m json.tool
}

validate_objective_coverage() {
  python3 - "$repo/$out" <<'PY'
import json, os, sys
data = json.load(open(sys.argv[1]))
errors, seen = [], set()
required = {"privacy-exclusion", "autonomous-merge", "source-shape", "backend-projection", "codegen-chain", "pdca-learning", "actions-receipt"}
kinds = {row.get("kind") for row in data.get("requirements", [])}
for row in data.get("requirements", []):
    if row["id"] in seen:
        errors.append(f"duplicate objective row {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "tracked":
        errors.append(f"{row['id']} must be tracked, not a completion claim")
    if not row.get("gate", "").startswith("gate:"):
        errors.append(f"{row['id']} missing gate reference")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            errors.append(f"{row['id']} missing evidence {item}")
if required - kinds:
    errors.append(f"missing objective kinds {sorted(required - kinds)}")
evidence = json.load(open("docs/generated/verification-evidence.json"))
backends = {row["backend"] for row in evidence.get("generated_backends", [])}
for backend in ("github-actions", "gitlab-ci", "local-makefile", "bazel", "source-shape"):
    if backend not in backends:
        errors.append(f"missing generated backend evidence {backend}")
codegen = json.load(open("docs/generated/verification-codegen.json"))
axes = {row["axis"] for row in codegen.get("axes", [])}
for axis in ("code", "docs", "schemas", "tests", "conformance", "github-actions", "release-pipelines"):
    if axis not in axes:
        errors.append(f"missing codegen axis {axis}")
if not data.get("closure_rules"):
    errors.append("objective coverage manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification objective coverage check ok")
PY
}

case "$mode" in
  generate)
    mkdir -p "$(dirname "$repo/$out")"
    generate > "$repo/$out"
    echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated objective coverage is stale: run scripts/verificationobjectivegen.sh generate" \
      "verification objective coverage generated output ok"
    validate_objective_coverage ;;
  *) echo "usage: scripts/verificationobjectivegen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
