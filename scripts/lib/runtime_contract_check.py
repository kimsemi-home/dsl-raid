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
    if result.returncode:
        add_error(result.stderr.strip())

def check_row(row):
    if row["id"] in seen:
        add_error(f"duplicate runtime contract surface {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "checked":
        add_error(f"{row['id']} must be checked")
    if not os.path.exists(row.get("fixture", "")):
        add_error(f"{row['id']} missing fixture")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            add_error(f"{row['id']} missing evidence {item}")
    if row["assertion"].startswith("stdout:"):
        assert_stdout(row, run(row["command"]))
    else:
        add_error(f"{row['id']} has unknown assertion {row['assertion']}")

for row in data.get("surfaces", []):
    check_row(row)

if not data.get("surfaces"):
    add_error("runtime contract manifest has no surfaces")
if not data.get("closure_rules"):
    add_error("runtime contract manifest has no rules")
if errors:
    print("\n".join(filter(None, errors)), file=sys.stderr)
    sys.exit(1)
print("verification runtime contract check ok")
