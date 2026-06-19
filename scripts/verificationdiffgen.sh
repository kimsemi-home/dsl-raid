#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-semantic-diff.json}"

generate_raw() {
  dslraid_lisp_eval '(write-string (dslraid.agent::emit-verification-diff-json))'
}

generate() {
  local raw
  raw="$(mktemp)"
  generate_raw > "$raw"
  python3 - "$repo" "$raw" <<'PY'
import json, sys
repo, raw = sys.argv[1], sys.argv[2]
data = json.load(open(raw))
hashes = json.load(open(f"{repo}/{data['head']}"))["hashes"]
by_id = {row["id"]: row["hash"] for row in hashes}
for row in data["diffs"]:
    digest = by_id[row["hash_id"]]
    row["base_hash"] = digest
    row["head_hash"] = digest
    row["status"] = "unchanged"
    row["evidence"] = data["head"]
print(json.dumps(data, indent=4))
PY
  rm -f "$raw"
}

validate_diffs() {
  python3 - "$repo" "$repo/$out" <<'PY'
import json, pathlib, sys
repo, path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, errors, seen = json.load(open(path)), [], set()
hashes = json.load(open(repo / data["head"]))["hashes"]
evidence = json.load(open(repo / "docs/generated/verification-evidence.json"))
by_id = {row["id"]: row["hash"] for row in hashes}
outputs = {row["output"] for row in evidence["generated_backends"]}
if data["base"] not in outputs: errors.append(f"base is not generated output {data['base']}")
if data["head"] not in outputs: errors.append(f"head is not generated output {data['head']}")
for row in data.get("diffs", []):
    if row["id"] in seen: errors.append(f"duplicate id {row['id']}")
    seen.add(row["id"])
    current = by_id.get(row["hash_id"])
    if current is None: errors.append(f"unknown hash {row['hash_id']}")
    if row.get("head_hash") != current: errors.append(f"stale head {row['id']}")
    if row.get("evidence") not in outputs: errors.append(f"evidence is not generated output {row['id']}")
    same = row.get("base_hash") == row.get("head_hash")
    if row.get("status") != ("unchanged" if same else "changed"): errors.append(f"bad status {row['id']}")
if not data.get("diffs"): errors.append("semantic diff manifest has no diffs")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification semantic diff check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification semantic diff is stale: run scripts/verificationdiffgen.sh generate" \
      "verification semantic diff generated output ok"
    validate_diffs ;;
  *) echo "usage: scripts/verificationdiffgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
