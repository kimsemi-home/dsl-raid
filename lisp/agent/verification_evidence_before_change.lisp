(in-package #:dslraid.agent)

(defparameter *verification-evidence-before-change-items*
  '(("change:generated-verification-graph" "routine"
     "agent:verification-daemon" "gate:quality"
     ("docs/generated/verification-evidence.json"
      "docs/generated/verification-sidecar.json")
     nil
     "Generated verification graph changes require linked evidence.")
    ("change:release-routing" "routine"
     "agent:verification-daemon" "gate:release"
     ("docs/generated/verification-orchestration.json"
      "docs/generated/verification-authority.json")
     nil
     "Release routing changes require authority and orchestration evidence.")
    ("change:emergency-patch" "emergency"
     "agent:incident-runner" "gate:incident"
     nil
     ("debt:evidence-follow-up")
     "Emergency changes may proceed only by creating evidence debt.")))

(defparameter *verification-evidence-before-change-rules*
  '(("evidence-before-change:linked" "Routine changes require evidence.")
    ("evidence-before-change:debt" "Emergency changes without evidence create debt.")
    ("evidence-before-change:authority" "Agents cannot authorize their own changes.")))

(defun emit-verification-evidence-before-change-json (&optional stream)
  "Emit evidence-before-change receipts for verification work."
  (let ((json (with-output-to-string (out)
                (write-verification-evidence-before-change out))))
    (if stream (write-string json stream) json)))

(defun write-verification-evidence-before-change (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationevidencebeforechangegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"evidence_before_change_profile\": \"linked-evidence-or-debt\",~%")
  (write-evidence-before-change-items out)
  (format out ",~%")
  (write-evidence-before-change-rules out)
  (format out "~%}~%"))

(defun write-evidence-before-change-items (out)
  (format out "  \"changes\": [~%")
  (loop for row in *verification-evidence-before-change-items*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-evidence-before-change-item out row))
  (format out "~%  ]"))

(defun write-evidence-before-change-item (out row)
  (destructuring-bind (id kind proposer authority evidence debt meaning) row
    (format out "    {\"id\": \"~A\", \"change_kind\": \"~A\", " id kind)
    (format out "\"proposed_by\": \"~A\", \"authority\": \"~A\", " proposer authority)
    (write-authority-list out "evidence" evidence)
    (format out ", ")
    (write-authority-list out "debt" debt)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-evidence-before-change-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-evidence-before-change-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationevidencebeforechangegen.sh check\"}")))
  (format out "~%  ]"))
