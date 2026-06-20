import json
import pathlib
import sys

from runtime_trace_assertions import collect_result


def load_json(path):
    return json.load(open(path, encoding="utf-8"))


def print_triples(triples):
    for triple in sorted(triples):
        print("\t".join(triple))


def main():
    if len(sys.argv) != 4:
        print("usage: runtime_trace_check.py repo manifest evidence", file=sys.stderr)
        sys.exit(2)
    repo, manifest, evidence = sys.argv[1], sys.argv[2], sys.argv[3]
    text = pathlib.Path(manifest).read_text(encoding="utf-8")
    errors, triples = collect_result(load_json(manifest), load_json(evidence), repo, text)
    if errors:
        print("\n".join(errors), file=sys.stderr)
        sys.exit(1)
    print_triples(triples)


if __name__ == "__main__":
    main()
