(in-package #:dslraid.agent)

(defparameter *verification-release-check-extra-commands*
  '("bash scripts/verificationautomergegen.sh check"
    "cargo run -p dslraid-cli -- artifact verify examples/runscope/runscope.raid.json"))
