(in-package #:dslraid.agent)

(defparameter *release-check-runtime-groups*
  '(("runtime" "runtime, merge, and remote action receipts"
     ("bash scripts/verificationruntimegen.sh check"
      "bash scripts/verificationrunmanifestgen.sh check"
      "bash scripts/verificationbootstrapgen.sh check"
      "bash scripts/verificationexperimentgen.sh check"
      "bash scripts/verificationmergegen.sh check"
      "bash scripts/verificationautomergegen.sh check"
      "bash scripts/verificationbranchgen.sh check"
      "bash scripts/verificationreceiptgen.sh check"
      "bash scripts/verificationsourcegen.sh check"
      "bash scripts/verificationobjectivegen.sh check"
      "bash scripts/verificationadrgen.sh check"
      "bash scripts/verificationparitygen.sh check"
      "bash scripts/verificationactionsgen.sh check"
      "bash scripts/verificationreleaseprovenancegen.sh check"
      "bash scripts/verificationincidentgen.sh check"))))
