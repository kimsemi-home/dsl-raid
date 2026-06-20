import json
import sys

from semantic_os_assertions import collect_semantic_os_errors


def load_json(path):
    return json.load(open(path, encoding="utf-8"))


def main():
    if len(sys.argv) != 3:
        print("usage: semantic_os_check.py manifest repo", file=sys.stderr)
        sys.exit(2)
    errors = collect_semantic_os_errors(load_json(sys.argv[1]), sys.argv[2])
    if errors:
        print("\n".join(errors), file=sys.stderr)
        sys.exit(1)
    print("verification semantic os check ok")


if __name__ == "__main__":
    main()
