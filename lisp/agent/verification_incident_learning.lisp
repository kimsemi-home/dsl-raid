(in-package #:dslraid.agent)

(defparameter *verification-incident-cycles*
  '(("incident:generated-drift-drill" "observation:generated-drift"
     ("docs/generated/verification-semantic-diff.json"
      "docs/generated/verification-conformance.json")
     "root-cause:ssot-defect-drill" "owner:verification"
     "update:generated-freshness-rule" "revalidate:next-quality-gate"
     ("prevention:add-generated-check" "prevention:record-semantic-hash")
     ("debt:verification-surface") "closed"
     "Generated drift closes only after evidence, owner, update, and recheck.")
    ("incident:release-gate-gap" "observation:release-gate-gap"
     ("docs/generated/verification-release-provenance.json"
      "docs/generated/verification-github-actions.json")
     "root-cause:release-provenance-gap" "owner:release"
     "update:release-provenance-gate" "revalidate:tag-pipeline"
     ("prevention:tag-bound-gate" "prevention:publish-permission-check")
     nil "closed"
     "Release incidents update promotion knowledge before authority returns.")))

(defparameter *verification-incident-rules*
  '(("incident-learning:evidence-linked" "Every incident cites generated evidence.")
    ("incident-learning:owner-required" "Every incident has a non-agent owner.")
    ("incident-learning:update-required" "Closure names a knowledge update.")
    ("incident-learning:revalidation-required" "Closure names revalidation.")))

(defun emit-verification-incident-json (&optional stream)
  "Emit incident learning receipts for Agent Cluster operations."
  (let ((json (with-output-to-string (out)
                (write-verification-incident out))))
    (if stream (write-string json stream) json)))

(defun write-verification-incident (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationincidentgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_incident_learning.lisp\",~%")
  (format out "  \"incident_learning_profile\": \"observe-learn-prevent\",~%")
  (write-incident-cycles out)
  (format out ",~%")
  (write-incident-rules out)
  (format out "~%}~%"))

(defun write-incident-cycles (out)
  (format out "  \"cycles\": [~%")
  (loop for row in *verification-incident-cycles*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-incident-cycle out row))
  (format out "~%  ]"))

(defun write-incident-cycle (out row)
  (destructuring-bind (id observation evidence cause owner update reval prevent debt status meaning) row
    (format out "    {\"id\": \"~A\", \"observation\": \"~A\", " id observation)
    (write-authority-list out "evidence" evidence)
    (format out ", \"root_cause\": \"~A\", \"owner\": \"~A\", " cause owner)
    (format out "\"knowledge_update\": \"~A\", " update)
    (format out "\"revalidation\": \"~A\", " reval)
    (write-authority-list out "prevention" prevent)
    (format out ", ")
    (write-authority-list out "debt" debt)
    (format out ", \"status\": \"~A\", \"meaning\": \"~A\"}" status meaning)))

(defun write-incident-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-incident-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationincidentgen.sh check\"}")))
  (format out "~%  ]"))
