import json
import pathlib
import sys

from failure_assertions import collect_failure_errors


def load_json(path):
    return json.load(open(path, encoding="utf-8"))


def main():
    if len(sys.argv) != 4:
        print("usage: failure_check.py manifest evidence repo", file=sys.stderr)
        sys.exit(2)
    text = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
    errors = collect_failure_errors(
        load_json(sys.argv[1]), load_json(sys.argv[2]), sys.argv[3], text
    )
    if errors:
        print("\n".join(errors), file=sys.stderr)
        sys.exit(1)
    print("verification failure conditions check ok")


if __name__ == "__main__":
    main()
