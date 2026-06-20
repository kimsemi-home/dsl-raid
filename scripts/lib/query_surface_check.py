import json
import os
import subprocess
import sys

data = json.load(open(sys.argv[1]))
errors = []
seen = set()

def add_error(message):
    errors.append(message)

def run_json(command):
    result = subprocess.run(["bash", "-lc", command], text=True, capture_output=True)
    if result.returncode:
        add_error(f"command failed: {command}\n{result.stderr}")
        return None
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        add_error(f"command did not return JSON: {command}: {exc}")
        return None

def assert_subject(value, subject):
    if not isinstance(value, list):
        add_error(f"{subject} assertion expected query list output")
        return
    subjects = {row.get("subject") for row in value}
    if subject not in subjects:
        add_error(f"query output missing {subject}")

def assert_lazy_empty(value):
    comp = value.get("composition", {})
    if comp.get("lazy") is not True or value.get("states") != []:
        add_error("diagnostics-only composition must stay lazy with no states")

def assert_lazy_focused(value):
    comp = value.get("composition", {})
    if comp.get("lazy") is not True or comp.get("focus") != "state:runtime.running":
        add_error("focused composition must keep lazy focus metadata")
    if comp.get("truncated") or not value.get("states"):
        add_error("focused composition must produce non-truncated focused states")

checks = {
    "compose-lazy-empty": assert_lazy_empty,
    "compose-lazy-focused": assert_lazy_focused,
}

for row in data.get("surfaces", []):
    if row["id"] in seen: add_error(f"duplicate query surface {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "checked": add_error(f"{row['id']} must be checked")
    if not os.path.exists(row.get("fixture", "")): add_error(f"{row['id']} missing fixture")
    for item in row.get("evidence", []):
        if not os.path.exists(item): add_error(f"{row['id']} missing evidence {item}")
    output = run_json(row["command"])
    if output is None:
        continue
    assertion = row["assertion"]
    if assertion.startswith("contains-subject:"):
        assert_subject(output, assertion.removeprefix("contains-subject:"))
    elif assertion in checks:
        checks[assertion](output)
    else:
        add_error(f"{row['id']} has unknown assertion {assertion}")

if not data.get("surfaces"):
    add_error("query surface manifest has no surfaces")
if not data.get("closure_rules"):
    add_error("query surface manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification query surface check ok")
