import json
import os
import sys
from objective_policy import (
    gate_evidence,
    required_axes,
    required_backends,
    required_kinds,
)

data = json.load(open(sys.argv[1]))
errors = []
seen = set()

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
if required_kinds - kinds:
    add_error(f"missing objective kinds {sorted(required_kinds - kinds)}")
evidence = json.load(open("docs/generated/verification-evidence.json"))
backends = {row["backend"] for row in evidence.get("generated_backends", [])}
for backend in required_backends:
    if backend not in backends:
        add_error(f"missing generated backend evidence {backend}")
codegen = json.load(open("docs/generated/verification-codegen.json"))
axes = {row["axis"] for row in codegen.get("axes", [])}
for axis in required_axes:
    if axis not in axes:
        add_error(f"missing codegen axis {axis}")
if not data.get("closure_rules"):
    add_error("objective coverage manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification objective coverage check ok")
