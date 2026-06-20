import json
import os
import sys

data = json.load(open(sys.argv[1]))
errors = []
seen = set()
required = {
    "privacy-exclusion", "autonomous-merge", "source-shape",
    "backend-projection", "actions-hardening", "release-pipeline",
    "codegen-chain", "pdca-learning", "evidence-governance",
    "actions-receipt", "query-lazy-surface", "learning-loop",
    "local-precommit",
}
gate_evidence = {
    "gate:privacy": ["docs/generated/verification-privacy.json"],
    "gate:merge-readiness": ["docs/generated/verification-merge-readiness.json"],
    "gate:source-shape": ["docs/generated/verification-source-shape.json"],
    "gate:precommit": ["docs/generated/verification-precommit-closure.json"],
    "gate:backend-parity": ["docs/generated/verification-backend-parity.json"],
    "gate:github-actions": ["docs/generated/verification-github-actions.json"],
    "gate:release": ["docs/generated/verification-release-provenance.json"],
    "gate:codegen": ["docs/generated/verification-codegen.json"],
    "gate:pdca": ["docs/generated/verification-pdca.json"],
    "gate:evidence-governance": ["docs/generated/verification-pruning.json"],
    "gate:learning-loop": ["docs/generated/verification-learning-loop.json"],
    "gate:actions-receipt": ["docs/generated/verification-actions-receipt.json"],
    "gate:query-surface": ["docs/generated/verification-query-surface.json"],
}

def add_error(message):
    errors.append(message)

def check_row(row):
    rid = row["id"]
    if rid in seen:
        add_error(f"duplicate objective row {rid}")
    seen.add(rid)
    if row.get("status") != "tracked":
        add_error(f"{rid} must be tracked, not a completion claim")
    gate = row.get("gate", "")
    if gate not in gate_evidence:
        add_error(f"{rid} missing generated manifest mapping for {gate}")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            add_error(f"{rid} missing evidence {item}")
    mapped = set(gate_evidence.get(gate, []))
    if not mapped.issubset(set(row.get("evidence", []))):
        add_error(f"{rid} gate {gate} lacks mapped evidence {sorted(mapped)}")

for row in data.get("requirements", []):
    check_row(row)

kinds = {row.get("kind") for row in data.get("requirements", [])}
if required - kinds:
    add_error(f"missing objective kinds {sorted(required - kinds)}")
evidence = json.load(open("docs/generated/verification-evidence.json"))
backends = {row["backend"] for row in evidence.get("generated_backends", [])}
for backend in ("github-actions", "gitlab-ci", "local-makefile", "bazel",
                "source-shape", "query-surface", "evidence-graph",
                "public-projection"):
    if backend not in backends:
        add_error(f"missing generated backend evidence {backend}")
codegen = json.load(open("docs/generated/verification-codegen.json"))
axes = {row["axis"] for row in codegen.get("axes", [])}
for axis in ("code", "docs", "schemas", "tests", "conformance",
             "github-actions", "release-pipelines"):
    if axis not in axes:
        add_error(f"missing codegen axis {axis}")
if not data.get("closure_rules"):
    add_error("objective coverage manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification objective coverage check ok")
