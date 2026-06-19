#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_dir/lib/generated.sh"
source "$script_dir/lib/lisp-runtime.sh"

dslraid_enter_repo

mode="${1:-check}"
out="${2:-docs/generated/verification-semantic-hash.json}"

generate_raw() {
  dslraid_lisp_eval \
    '(write-string (dslraid.agent::emit-verification-semantic-json))'
}

generate() {
  local raw
  raw="$(mktemp)"
  generate_raw > "$raw"
  python3 - "$repo" "$raw" <<'PY'
import hashlib, json, sys
repo, raw = sys.argv[1], sys.argv[2]
data = json.load(open(raw))
for row in data["hashes"]:
    with open(f"{repo}/{row['source']}") as handle:
        source = json.load(handle)
    payload = {field: source[field] for field in row["fields"]}
    canonical = json.dumps(payload, sort_keys=True, separators=(",", ":"))
    row["hash"] = hashlib.sha256(canonical.encode()).hexdigest()
print(json.dumps(data, indent=4))
PY
  rm -f "$raw"
}

validate_hashes() {
  python3 - "$repo" "$repo/$out" <<'PY'
import hashlib, json, pathlib, sys
repo, path = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
data, errors, seen = json.load(open(path)), [], set()
evidence = json.load(open(repo / "docs/generated/verification-evidence.json"))
outputs = {row["output"] for row in evidence["generated_backends"]}
for row in data.get("hashes", []):
    if row["id"] in seen: errors.append(f"duplicate id {row['id']}")
    seen.add(row["id"])
    source_path = repo / row["source"]
    if not source_path.exists(): errors.append(f"missing source {row['source']}")
    if row["source"] not in outputs: errors.append(f"source is not generated output {row['source']}")
    source = json.load(open(source_path))
    payload = {field: source[field] for field in row["fields"]}
    canonical = json.dumps(payload, sort_keys=True, separators=(",", ":"))
    digest = hashlib.sha256(canonical.encode()).hexdigest()
    if row.get("hash") != digest: errors.append(f"stale hash {row['id']}")
if not data.get("hashes"): errors.append("semantic hash manifest has no hashes")
if errors:
    print("\n".join(errors), file=sys.stderr)
    sys.exit(1)
print("verification semantic hash check ok")
PY
}

case "$mode" in
  generate) mkdir -p "$(dirname "$repo/$out")"; generate > "$repo/$out"; echo "generated $out" ;;
  check)
    dslraid_generated_check "$out" \
      "generated verification semantic hash is stale: run scripts/verificationsemanticgen.sh generate" \
      "verification semantic hash generated output ok"
    validate_hashes ;;
  *) echo "usage: scripts/verificationsemanticgen.sh [generate|check] [out]" >&2; exit 2 ;;
esac
