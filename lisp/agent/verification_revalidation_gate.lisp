(in-package #:dslraid.agent)

(defparameter *verification-revalidation-gates*
  '(("revalidation:verification-ssot" "versioned-ssot:verification-graph"
     "valid" "2026-07-17T00:00:00Z" "normal-authority"
     ("none")
     ("release-check" "quality-gate" "generated-artifact-refresh")
     "owner:verification"
     ("docs/generated/verification-versioned-ssot.json"
      "docs/generated/verification-conformance.json")
     "Valid SSOT can keep routine generated authority.")
    ("revalidation:evidence-quality" "evidence-quality:generated-evidence"
     "due-soon" "2026-07-24T00:00:00Z" "review-required"
     ("high-risk-automation")
     ("evidence-refresh" "review" "revalidation-work")
     "owner:evidence"
     ("docs/generated/verification-evidence-quality.json"
      "docs/generated/verification-pruning.json")
     "Due-soon evidence needs review before expanded automation.")
    ("revalidation:security-boundary" "security-audit:permission-change"
     "grace" "2026-07-10T00:00:00Z" "authority-limited"
     ("major-ontology-change" "security-boundary-change" "automatic-approval-expansion")
     ("read" "incident-response" "risk-reducing-change")
     "owner:security"
     ("docs/generated/verification-security-audit.json"
      "docs/generated/verification-authority.json")
     "Grace state permits risk reduction but blocks authority expansion.")))

(defparameter *verification-revalidation-rules*
  '(("revalidation:date-is-gate" "Revalidation date changes authority.")
    ("revalidation:expired-limits-authority" "Expired or frozen states block expansion.")
    ("revalidation:evidence-linked" "Revalidation decisions cite generated evidence.")
    ("revalidation:owner-required" "Revalidation has a non-agent owner.")))

(defun emit-verification-revalidation-gate-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-revalidation-gate out))))
    (if stream (write-string json stream) json)))

(defun write-verification-revalidation-gate (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationrevalidationgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_revalidation_gate.lisp\",~%")
  (format out "  \"revalidation_profile\": \"authority-gate\",~%")
  (write-revalidation-gates out) (format out ",~%") (write-revalidation-rules out)
  (format out "~%}~%"))

(defun write-revalidation-gates (out)
  (format out "  \"gates\": [~%")
  (loop for row in *verification-revalidation-gates* for first = t then nil
        do (unless first (format out ",~%")) (write-revalidation-gate out row))
  (format out "~%  ]"))

(defun write-revalidation-gate (out row)
  (destructuring-bind (id subject status due effect blocks allowed owner evidence meaning) row
    (format out "    {\"id\": \"~A\", \"gated_subject\": \"~A\", " id subject)
    (format out "\"status\": \"~A\", \"due_at\": \"~A\", " status due)
    (format out "\"authority_effect\": \"~A\", " effect)
    (write-authority-list out "blocks" blocks) (format out ", ")
    (write-authority-list out "allowed" allowed) (format out ", ")
    (format out "\"owner\": \"~A\", " owner)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-revalidation-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-revalidation-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationrevalidationgen.sh check\"}")))
  (format out "~%  ]"))
