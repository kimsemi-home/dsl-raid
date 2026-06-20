(in-package #:dslraid.agent)

(defparameter *verification-operating-loop-tail-stages*
  '((9 "operating-loop:change-candidate" "change-candidate"
     "lisp/agent/verification_evidence_before_change.lisp"
     "docs/generated/verification-evidence-before-change.json"
     "bash scripts/verificationevidencebeforechangegen.sh check"
     "stdout:verification evidence-before-change check ok"
     "gate:evidence-before-change" ("docs/generated/verification-evidence-before-change.json")
     "Change candidates require evidence before change.")
    (10 "operating-loop:external-confidence" "external-confidence-assessment"
     "lisp/agent/verification_confidence.lisp" "docs/generated/verification-confidence.json"
     "bash scripts/verificationconfidencegen.sh check" "stdout:verification confidence check ok"
     "gate:external-confidence" ("docs/generated/verification-confidence.json")
     "Confidence is externally assessed.")
    (11 "operating-loop:authority-gate" "authority-gate"
     "lisp/agent/verification_authority.lisp" "docs/generated/verification-authority.json"
     "bash scripts/verificationauthoritygen.sh check" "stdout:verification authority check ok"
     "gate:authority" ("docs/generated/verification-authority.json")
     "Authority gates decide whether change may proceed.")
    (12 "operating-loop:change" "change"
     "lisp/agent/verification_merge_readiness.lisp"
     "docs/generated/verification-merge-readiness.json"
     "bash scripts/verificationmergegen.sh check" "stdout:verification merge readiness check ok"
     "gate:merge-readiness" ("docs/generated/verification-merge-readiness.json")
     "Change is represented as merge-ready evidence.")
    (13 "operating-loop:verification-evidence" "verification-evidence"
     "lisp/agent/verification_semantic_hash.lisp"
     "docs/generated/verification-semantic-hash.json"
     "bash scripts/verificationsemanticgen.sh check" "stdout:verification semantic hash check ok"
     "gate:semantic-hash" ("docs/generated/verification-semantic-hash.json")
     "Verification evidence records semantic hashes.")
    (14 "operating-loop:conformance-evidence" "conformance-evidence"
     "lisp/agent/verification_conformance.lisp" "docs/generated/verification-conformance.json"
     "bash scripts/verificationconformancegen.sh check" "stdout:verification conformance generated output ok"
     "gate:conformance" ("docs/generated/verification-conformance.json")
     "Conformance evidence validates the changed target.")
    (15 "operating-loop:evidence-graph-update" "evidence-graph-update"
     "lisp/agent/verification_evidence.lisp" "docs/generated/verification-evidence.json"
     "bash scripts/verificationevidencegen.sh check" "stdout:verification evidence generated output ok"
     "gate:evidence-graph" ("docs/generated/verification-evidence.json")
     "Evidence graph updates become durable memory.")))

(defparameter *verification-operating-loop-rules*
  '(("operating-loop:ordered" "Operating stages are strictly ordered.")
    ("operating-loop:evidence-backed" "Every stage has command-backed evidence.")
    ("operating-loop:knowledge-closed" "The loop closes only after knowledge update.")))

(defparameter *verification-operating-loop-stages*
  (append *verification-operating-loop-core-stages*
          *verification-operating-loop-tail-stages*))
