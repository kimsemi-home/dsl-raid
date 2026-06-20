(in-package #:dslraid.agent)

(defparameter *release-check-artifact-groups*
  '(("artifacts" "generated code, docs, index, and artifact freshness"
     ("bash scripts/lisp-rustgen.sh check"
      "bash scripts/verificationdocgen.sh check"
      "bash scripts/verificationcodegengen.sh check"
      "bash scripts/gendocindex.sh check"
      "cargo run -p dslraid-cli -- artifact verify examples/runscope/runscope.raid.json"))))
