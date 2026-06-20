import json
import os
import subprocess
import sys

data = json.load(open(sys.argv[1]))
errors = []
seen = set()

def add_error(message):
    errors.append(message)

def run(command):
    return subprocess.run(["bash", "-lc", command], text=True, capture_output=True)

def assert_stdout(row, result):
    expected = row["assertion"].removeprefix("stdout:")
    if result.returncode or expected not in result.stdout:
        add_error(f"{row['id']} expected stdout {expected!r}")

def check_row(row):
    if row["id"] in seen:
        add_error(f"duplicate language expansion surface {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "checked":
        add_error(f"{row['id']} must be checked")
    if not os.path.exists(row.get("fixture", "")):
        add_error(f"{row['id']} missing fixture")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            add_error(f"{row['id']} missing evidence {item}")
    if not row["assertion"].startswith("stdout:"):
        add_error(f"{row['id']} has unknown assertion {row['assertion']}")
        return
    assert_stdout(row, run(row["command"]))

for row in data.get("surfaces", []):
    check_row(row)

if not data.get("surfaces"):
    add_error("language expansion manifest has no surfaces")
if not data.get("closure_rules"):
    add_error("language expansion manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification language expansion check ok")
