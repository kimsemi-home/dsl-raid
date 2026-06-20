import json
import os
import sys

from workflow_lineage_policy import expected_surfaces

manifest, evidence = [json.load(open(path, encoding="utf-8")) for path in sys.argv[1:3]]
known_nodes = {row["id"] for row in evidence["verification_nodes"]}
generated = {row["output"]: row for row in evidence["generated_backends"]}
errors = []
seen_ids, seen_surfaces = set(), set()
add = errors.append

def check_unique(row):
    rid = row["id"]
    surface = row["surface"]
    if rid in seen_ids:
        add(f"duplicate lineage {rid}")
    if surface in seen_surfaces:
        add(f"duplicate surface {surface}")
    seen_ids.add(rid)
    seen_surfaces.add(surface)

def check_expected(row):
    expected = expected_surfaces.get(row["surface"])
    if not expected:
        add(f"{row['id']} has unexpected surface {row['surface']}")
        return
    artifact, nodes = expected
    if row["artifact"] != artifact:
        add(f"{row['id']} artifact mismatch")
    if tuple(row["graph_nodes"]) != tuple(nodes):
        add(f"{row['id']} graph node order mismatch")

def check_paths(row):
    for field in ("artifact", "generator"):
        if not os.path.exists(row[field]):
            add(f"{row['id']} missing {field} {row[field]}")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            add(f"{row['id']} missing evidence {item}")

def check_backend(row):
    backend = generated.get(row["artifact"])
    if not backend:
        add(f"{row['id']} artifact is not a generated backend")
        return
    if backend["generator"] != row["generator"]:
        add(f"{row['id']} generator mismatch")
    if backend["check"] != row["check"]:
        add(f"{row['id']} check mismatch")

def check_row(row):
    check_unique(row)
    check_expected(row)
    check_paths(row)
    check_backend(row)
    if row.get("status") != "generated":
        add(f"{row['id']} status must be generated")
    if not set(row["graph_nodes"]).issubset(known_nodes):
        add(f"{row['id']} references unknown graph node")
    if row["check"] != f"{row['generator']} check":
        add(f"{row['id']} check must match generator")

for row in manifest.get("lineages", []):
    check_row(row)

if seen_surfaces != set(expected_surfaces):
    add(f"workflow lineage surface set mismatch: {sorted(seen_surfaces)}")
if not manifest.get("closure_rules"):
    add("workflow lineage has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification workflow lineage check ok")
