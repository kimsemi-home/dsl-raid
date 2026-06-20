import json
import os
import sys

data = json.load(open(sys.argv[1]))
errors = []
seen = set()

def add_error(message):
    errors.append(message)

def read(path):
    return open(path, encoding="utf-8").read()

def require_text(row, text, needle, label):
    if needle not in text:
        add_error(f"{row['id']} missing {label}: {needle}")

def check_projection(row):
    if row["id"] in seen:
        add_error(f"duplicate execution projection {row['id']}")
    seen.add(row["id"])
    output = row.get("output", "")
    generator = row.get("generator", "")
    if not os.path.exists(output):
        add_error(f"{row['id']} missing output {output}")
        return
    if not os.path.exists(generator):
        add_error(f"{row['id']} missing generator {generator}")
    text = read(output)
    for node in row.get("graph_nodes", []):
        require_text(row, text, node, "graph node")
    for command in row.get("command_probes", []):
        require_text(row, text, command, "command probe")
    expected_check = f"{generator} check"
    if row.get("check") != expected_check:
        add_error(f"{row['id']} check must be {expected_check}")

for row in data.get("projections", []):
    check_projection(row)

if len(data.get("projections", [])) != 4:
    add_error("execution projection requires four backend projections")
if not data.get("closure_rules"):
    add_error("execution projection manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification execution projection check ok")
