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

def assert_stdout(row, result, text):
    if result.returncode or text not in result.stdout:
        add_error(f"{row['id']} expected stdout {text!r}")

def assert_json_version(row, result, version):
    if result.returncode:
        add_error(f"{row['id']} failed: {result.stderr}")
        return
    try:
        output = json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        add_error(f"{row['id']} did not emit JSON: {exc}")
        return
    if output.get("ir_version") != version:
        add_error(f"{row['id']} emitted version {output.get('ir_version')}")

def assert_failure(row, result, text):
    if result.returncode == 0 or text not in result.stderr:
        add_error(f"{row['id']} expected failure containing {text!r}")

def check_assertion(row, result):
    assertion = row["assertion"]
    if assertion.startswith("stdout:"):
        assert_stdout(row, result, assertion.removeprefix("stdout:"))
    elif assertion.startswith("json-ir-version:"):
        assert_json_version(row, result, assertion.removeprefix("json-ir-version:"))
    elif assertion.startswith("fails:"):
        assert_failure(row, result, assertion.removeprefix("fails:"))
    else:
        add_error(f"{row['id']} has unknown assertion {assertion}")

for row in data.get("surfaces", []):
    if row["id"] in seen: add_error(f"duplicate migration surface {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "checked": add_error(f"{row['id']} must be checked")
    if not os.path.exists(row.get("fixture", "")): add_error(f"{row['id']} missing fixture")
    for item in row.get("evidence", []):
        if not os.path.exists(item): add_error(f"{row['id']} missing evidence {item}")
    check_assertion(row, run(row["command"]))

if not data.get("surfaces"):
    add_error("migration surface manifest has no surfaces")
if not data.get("closure_rules"):
    add_error("migration surface manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification migration surface check ok")
