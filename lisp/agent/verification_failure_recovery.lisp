(in-package #:dslraid.agent)

(defparameter *verification-failure-recoveries*
  '(("failure-recovery:control-plane-manifest"
     "failure:control-plane-manifest" "response:create-manifest"
     "root-cause:release-provenance-gap" "incident:release-gate-gap"
     "update:release-provenance-gate" "revalidate:tag-pipeline"
     "gate:release" ("docs/generated/verification-failure-conditions.json"
                     "docs/generated/verification-incident-learning.json"
                     "docs/generated/verification-learning-loop.json"
                     "docs/generated/verification-release-provenance.json")
     "release-blocked-until-revalidated"
     "Control-plane failure closes only after release knowledge is updated.")
    ("failure-recovery:evidence-quality-stale"
     "failure:evidence-quality-stale" "response:reassess-quality"
     "root-cause:ssot-defect-drill" "incident:generated-drift-drill"
     "update:generated-freshness-rule" "revalidate:quality-gate"
     "gate:evidence-quality" ("docs/generated/verification-failure-conditions.json"
                              "docs/generated/verification-evidence-quality.json"
                              "docs/generated/verification-semantic-diff.json"
                              "docs/generated/verification-learning-loop.json")
     "release-blocked-until-revalidated"
     "Stale evidence returns to authority only through quality revalidation.")
    ("failure-recovery:feedback-open"
     "failure:feedback-open" "response:assign-owner"
     "root-cause:ssot-defect-drill" "incident:claim-confidence-gap"
     "update:confidence-ceiling-rule" "revalidate:actions-receipt"
     "gate:feedback" ("docs/generated/verification-failure-conditions.json"
                      "docs/generated/verification-feedback.json"
                      "docs/generated/verification-confidence.json"
                      "docs/generated/verification-actions-receipt.json")
     "learning-open"
     "Open feedback remains visible until ownership and learning are explicit.")))

(defparameter *verification-failure-recovery-rules*
  '(("failure-recovery:known-failure" "Each recovery references a known failure.")
    ("failure-recovery:response-matches" "Recovery response matches the failure.")
    ("failure-recovery:learning-linked" "Recovery names incident, update, and recheck.")
    ("failure-recovery:evidence-linked" "Recovery cites generated evidence.")))

(defun emit-verification-failure-recovery-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-failure-recovery out))))
    (if stream (write-string json stream) json)))

(defun write-verification-failure-recovery (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationfailurerecoverygen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_failure_recovery.lisp\",~%")
  (format out "  \"failure_recovery_profile\": \"failure-to-learning\",~%")
  (write-failure-recoveries out) (format out ",~%")
  (write-failure-recovery-rules out) (format out "~%}~%"))

(defun write-failure-recoveries (out)
  (format out "  \"recoveries\": [~%")
  (loop for row in *verification-failure-recoveries* for first = t then nil
        do (unless first (format out ",~%")) (write-failure-recovery out row))
  (format out "~%  ]"))

(defun write-failure-recovery (out row)
  (destructuring-bind (id failure response cause incident update reval gate evidence status meaning) row
    (format out "    {\"id\": \"~A\", \"failure\": \"~A\", " id failure)
    (format out "\"response\": \"~A\", \"root_cause\": \"~A\", " response cause)
    (format out "\"incident\": \"~A\", \"learning_update\": \"~A\", " incident update)
    (format out "\"revalidation\": \"~A\", \"release_gate\": \"~A\", " reval gate) (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"~A\", \"meaning\": \"~A\"}" status meaning)))

(defun write-failure-recovery-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-failure-recovery-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationfailurerecoverygen.sh check\"}")))
  (format out "~%  ]"))
