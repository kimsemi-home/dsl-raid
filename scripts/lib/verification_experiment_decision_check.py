import json
import pathlib
import sys

from verification_experiment_decision_assertions import collect_errors


def load_json(path):
    return json.load(open(path, encoding="utf-8"))


def main():
    if len(sys.argv) != 4:
        print("usage: verification_experiment_decision_check.py repo manifest experiments",
              file=sys.stderr)
        sys.exit(2)
    repo, manifest, experiments = sys.argv[1], sys.argv[2], sys.argv[3]
    text = pathlib.Path(manifest).read_text(encoding="utf-8")
    errors = collect_errors(load_json(manifest), load_json(experiments), repo, text)
    if errors:
        print("\n".join(errors), file=sys.stderr)
        sys.exit(1)
    print("verification experiment decision check ok")


if __name__ == "__main__":
    main()
