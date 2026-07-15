(in-package #:dslraid.agent)

(defun write-ci-workflow-lint-job (out)
  (write-ci-lines
   out
   '("  workflow-lint:"
     "    name: Workflow lint"
     "    runs-on: ubuntu-latest"
     "    steps:"
     "      - uses: actions/checkout@v6"
     ""
     "      - uses: actions/setup-go@v6"
     "        with:"
     "          go-version: 1.26.5"
     "          cache: false"
     ""
     "      - name: Install actionlint"
     "        run: go install github.com/rhysd/actionlint/cmd/actionlint@v1.7.12"
     ""
     "      - name: Lint GitHub Actions workflows"
     "        run: |"
     "          actionlint_bin=\"$(go env GOPATH)/bin/actionlint\""
     "          \"$actionlint_bin\""
     "")))

(defun write-ci-schemas-job (out)
  (write-ci-lines
   out
   '("  schemas:"
     "    name: Schemas and fixtures"
     "    runs-on: ubuntu-latest"
     "    steps:"
     "      - uses: actions/checkout@v6"
     ""
     "      - name: Validate JSON syntax"
     "        run: |"
     "          set -euo pipefail"
     "          find schemas examples -name '*.json' -print0 |"
     "            while IFS= read -r -d '' file; do"
     "              python3 -m json.tool \"$file\" >/dev/null"
     "            done"
     "")))
