import json
import sys

from release_provenance_assertions import collect_release_provenance_errors


def load_json(path):
    return json.load(open(path, encoding="utf-8"))


def main():
    if len(sys.argv) != 3:
        print("usage: release_provenance_check.py manifest actions", file=sys.stderr)
        sys.exit(2)
    manifest = load_json(sys.argv[1])
    actions = load_json(sys.argv[2])
    errors = collect_release_provenance_errors(manifest, actions)
    if errors:
        print("\n".join(errors), file=sys.stderr)
        sys.exit(1)
    print("verification release provenance check ok")


if __name__ == "__main__":
    main()
