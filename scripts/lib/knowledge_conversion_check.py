import json
import sys

from knowledge_conversion_assertions import collect_knowledge_conversion_errors


def load_json(path):
    return json.load(open(path, encoding="utf-8"))


def main():
    if len(sys.argv) != 3:
        print("usage: knowledge_conversion_check.py manifest repo", file=sys.stderr)
        sys.exit(2)
    errors = collect_knowledge_conversion_errors(load_json(sys.argv[1]), sys.argv[2])
    if errors:
        print("\n".join(errors), file=sys.stderr)
        sys.exit(1)
    print("verification knowledge conversion check ok")


if __name__ == "__main__":
    main()
