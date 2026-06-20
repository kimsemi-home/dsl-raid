(in-package #:dslraid.agent)

(defparameter *verification-abort-evidence-bundles*
  '(("abort:lease-timeout" "lease:verification-daemon" "decision:abort"
     "bundle:evidence-retained"
     ("docs/generated/verification-evidence.json"
      "docs/generated/verification-quality-closure.json")
     ("claim:fresh-artifacts" "claim:quality-gate")
     "docs/generated/verification-evidence.json" "commit-blocked"
     ("docs/generated/verification-lease.json"
      "docs/generated/verification-conformance.json")
     "Expired work keeps evidence and rechecks affected claims.")
    ("abort:stale-artifact" "lease:release-check" "decision:abort"
     "bundle:artifact-classified"
     ("generated/runtime_fsm.rs" "generated/runtime_fsm.go")
     ("claim:artifact-freshness")
     "docs/generated/verification-evidence.json" "commit-blocked"
     ("examples/runscope/runscope.lock.json"
      "docs/generated/verification-release-provenance.json")
     "Stale generated artifacts are classified before retry.")))

(defparameter *verification-abort-evidence-rules*
  '(("abort-evidence:bundle-required" "Abort creates an evidence bundle.")
    ("abort-evidence:artifact-classified" "Intermediate artifacts are classified.")
    ("abort-evidence:claim-recheck" "Affected claims are rechecked before retry.")
    ("abort-evidence:memory-retained" "Abort blocks commit but keeps memory.")))

(defun emit-verification-abort-evidence-json (&optional stream)
  "Emit abort evidence bundles for stopped verification work."
  (let ((json (with-output-to-string (out)
                (write-verification-abort-evidence out))))
    (if stream (write-string json stream) json)))

(defun write-verification-abort-evidence (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationabortgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_abort_evidence.lisp\",~%")
  (format out "  \"abort_profile\": \"evidence-retaining-stop\",~%")
  (write-abort-evidence-bundles out)
  (format out ",~%")
  (write-abort-evidence-rules out)
  (format out "~%}~%"))

(defun write-abort-evidence-bundles (out)
  (format out "  \"bundles\": [~%")
  (loop for row in *verification-abort-evidence-bundles*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-abort-evidence-bundle out row))
  (format out "~%  ]"))

(defun write-abort-evidence-bundle (out row)
  (destructuring-bind (id lease decision bundle artifacts claims update effect evidence meaning) row
    (format out "    {\"id\": \"~A\", \"lease\": \"~A\", " id lease)
    (format out "\"decision\": \"~A\", \"bundle\": \"~A\", " decision bundle)
    (write-authority-list out "classified_artifacts" artifacts)
    (format out ", ")
    (write-authority-list out "recheck_claims" claims)
    (format out ", \"evidence_update\": \"~A\", " update)
    (format out "\"authority_effect\": \"~A\", " effect)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-abort-evidence-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-abort-evidence-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationabortgen.sh check\"}")))
  (format out "~%  ]"))
