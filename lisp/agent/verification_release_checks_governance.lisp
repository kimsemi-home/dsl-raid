(in-package #:dslraid.agent)

(defparameter *release-check-governance-groups*
  '(("governance" "authority, review, and control-plane checks"
     ("bash scripts/verificationauthoritygen.sh check"
      "bash scripts/verificationaccessgen.sh check"
      "bash scripts/verificationreasoninggen.sh check"
      "bash scripts/verificationreliabilitygen.sh check"
      "bash scripts/verificationagreementgen.sh check"
      "bash scripts/verificationadversarialgen.sh check"
      "bash scripts/verificationevidencequalitygen.sh check"
      "bash scripts/verificationleasegen.sh check"
      "bash scripts/verificationabortgen.sh check"
      "bash scripts/verificationreviewgen.sh check"
      "bash scripts/verificationfeedbackgen.sh check"
      "bash scripts/verificationquarantinegen.sh check"
      "bash scripts/verificationquarantinereleasegen.sh check"
      "bash scripts/verificationconfidencegen.sh check"
      "bash scripts/verificationconfidencedecisiongen.sh check"
      "bash scripts/verificationsidecargen.sh check"
      "bash scripts/verificationorchestrationgen.sh check"
      "bash scripts/verificationcontrolgen.sh check"
      "bash scripts/verificationprovidergen.sh check"))))
