import json
import pathlib
import sys

from evidence_graph_assertions import collect_errors


def load_json(path):
    return json.load(open(path, encoding="utf-8"))


def main():
    if len(sys.argv) != 3:
        print("usage: evidence_graph_check.py manifest repo", file=sys.stderr)
        sys.exit(2)
    manifest, repo = sys.argv[1], sys.argv[2]
    text = pathlib.Path(manifest).read_text(encoding="utf-8")
    errors = collect_errors(load_json(manifest), repo, text)
    if errors:
        print("\n".join(errors), file=sys.stderr)
        sys.exit(1)
    print("verification evidence graph check ok")


if __name__ == "__main__":
    main()
