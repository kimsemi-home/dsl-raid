import json
import pathlib
import sys

from verification_feedback_assertions import collect_errors


def load_json(path):
    return json.load(open(path, encoding="utf-8"))


def main():
    if len(sys.argv) != 3:
        print("usage: verification_feedback_check.py manifest evidence", file=sys.stderr)
        sys.exit(2)
    manifest, evidence = sys.argv[1], sys.argv[2]
    text = pathlib.Path(manifest).read_text(encoding="utf-8")
    errors = collect_errors(load_json(manifest), load_json(evidence), text)
    if errors:
        print("\n".join(errors), file=sys.stderr)
        sys.exit(1)
    print("verification feedback closure check ok")


if __name__ == "__main__":
    main()
