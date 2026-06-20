import json
import os
import subprocess
import sys

data = json.load(open(sys.argv[1]))
errors = []
seen = set()
required = {"line-budget", "surface-boundary", "ssot-boundary", "generated-ownership", "provider-registry"}

def add_error(message):
    errors.append(message)

def require_text(path, text, label):
    if text not in open(path).read():
        add_error(f"{path} missing {label}")

for row in data.get("budgets", []):
    if row["id"] in seen:
        add_error(f"duplicate source shape budget {row['id']}")
    seen.add(row["id"])
    if row.get("status") != "required":
        add_error(f"{row['id']} must be required")
    for item in row.get("evidence", []):
        if not os.path.exists(item):
            add_error(f"{row['id']} missing evidence {item}")

kinds = {row.get("kind") for row in data.get("budgets", [])}
if required - kinds:
    add_error(f"missing source shape kinds {sorted(required - kinds)}")

line = next((r for r in data.get("budgets", []) if r["kind"] == "line-budget"), {})
if line.get("limit") != "75" or "check-source-lines.sh" not in line.get("command", ""):
    add_error("line budget must require scripts/check-source-lines.sh at limit 75")

quality_files = ["crates/dslraid-cli/src/commands/quality/lisp/scripts.rs"]
for root, _, files in os.walk("crates/dslraid-cli/src/commands/quality/lisp/scripts"):
    quality_files += [os.path.join(root, name) for name in files if name.endswith(".rs")]
quality_text = "\n".join(open(path).read() for path in quality_files)
if "scripts/verificationsourcegen.sh" not in quality_text:
    add_error("quality command must run source shape verification")

require_text(".github/workflows/ci.yml", "scripts/check-source-lines.sh", "line budget check")
require_text("lisp/dslraid.asd", "verification_semantic_hash_registry_foundation", "hash provider")
require_text("lisp/dslraid.asd", "verification_semantic_diff_registry_foundation", "diff provider")

for path in ("lisp/agent/verification_semantic_hash_registry.lisp",
             "lisp/agent/verification_semantic_diff_registry.lisp"):
    text = open(path).read()
    if "'((" in text or "docs/generated/" in text:
        add_error(f"{path} must stay facade-only")

if not data.get("closure_rules"):
    add_error("source shape manifest has no rules")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
subprocess.run(["bash", "scripts/check-source-lines.sh"], check=True)
print("verification source shape check ok")
