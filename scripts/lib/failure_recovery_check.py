import json
import pathlib
import sys

from failure_recovery_assertions import collect_recovery_errors


def load_json(path):
    return json.load(open(path, encoding="utf-8"))


def main():
    if len(sys.argv) != 7:
        print("usage: failure_recovery_check.py manifest failure incident learning evidence repo", file=sys.stderr)
        sys.exit(2)
    text = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
    sources = [load_json(path) for path in sys.argv[2:6]]
    errors = collect_recovery_errors(load_json(sys.argv[1]), sources, sys.argv[6], text)
    if errors:
        print("\n".join(errors), file=sys.stderr)
        sys.exit(1)
    print("verification failure recovery check ok")


if __name__ == "__main__":
    main()
