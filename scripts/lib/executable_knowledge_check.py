import json
import sys

from executable_knowledge_assertions import collect_executable_knowledge_errors


def load_json(path):
    return json.load(open(path, encoding="utf-8"))


def main():
    if len(sys.argv) != 3:
        print("usage: executable_knowledge_check.py manifest repo", file=sys.stderr)
        sys.exit(2)
    errors = collect_executable_knowledge_errors(load_json(sys.argv[1]), sys.argv[2])
    if errors:
        print("\n".join(errors), file=sys.stderr)
        sys.exit(1)
    print("verification executable knowledge check ok")


if __name__ == "__main__":
    main()
