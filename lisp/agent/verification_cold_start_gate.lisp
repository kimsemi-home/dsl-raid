(in-package #:dslraid.agent)

(defparameter *verification-cold-start-gates*
  '(("cold-start:candidate" "agent:candidate" "candidate" "proposal-only"
     ("read-public-ir" "draft-claim")
     ("commit-artifact" "approve-authority" "production-change" "major-ontology-change")
     ("sidecar-verification" "reliability-history" "human-review")
     "owner:review"
     ("docs/generated/verification-reliability.json"
      "docs/generated/verification-sidecar.json")
     "Candidate agents can propose but cannot change production authority.")
    ("cold-start:shadow" "agent:shadow" "shadow" "shadow-only"
     ("shadow-run" "compare-output")
     ("write-artifact" "merge" "release")
     ("agreement-review" "orchestration-receipt" "adversarial-review")
     "owner:orchestration"
     ("docs/generated/verification-orchestration.json"
      "docs/generated/verification-agreement.json")
     "Shadow agents compare decisions without changing operational state.")
    ("cold-start:bounded" "agent:quality-runner" "bounded" "bounded-operation"
     ("quality-gate" "generated-check")
     ("security-boundary-change" "major-ontology-change")
     ("revalidation-gate" "security-audit" "backup-steward-review")
     "owner:security"
     ("docs/generated/verification-revalidation-gate.json"
      "docs/generated/verification-security-audit.json")
     "Bounded agents keep narrow automation but cannot expand sensitive authority.")))

(defparameter *verification-cold-start-rules*
  '(("cold-start:no-production-first" "Cold-start agents cannot make production changes.")
    ("cold-start:no-self-promotion" "Promotion owner is not the candidate agent.")
    ("cold-start:evidence-required" "Promotion requires generated evidence.")
    ("cold-start:blocked-actions-explicit" "Restricted authority lists blocked actions.")))

(defun emit-verification-cold-start-gate-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-cold-start-gate out))))
    (if stream (write-string json stream) json)))

(defun write-verification-cold-start-gate (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationcoldstartgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_cold_start_gate.lisp\",~%")
  (format out "  \"cold_start_profile\": \"authority-onboarding\",~%")
  (write-cold-start-gates out)
  (format out ",~%")
  (write-cold-start-rules out)
  (format out "~%}~%"))

(defun write-cold-start-gates (out)
  (format out "  \"gates\": [~%")
  (loop for row in *verification-cold-start-gates* for first = t then nil
        do (unless first (format out ",~%")) (write-cold-start-gate out row))
  (format out "~%  ]"))

(defun write-cold-start-gate (out row)
  (destructuring-bind (id agent stage effect allowed blocked requires owner evidence meaning) row
    (format out "    {\"id\": \"~A\", \"agent\": \"~A\", " id agent)
    (format out "\"stage\": \"~A\", \"authority_effect\": \"~A\", " stage effect)
    (write-authority-list out "allowed" allowed) (format out ", ")
    (write-authority-list out "blocked" blocked) (format out ", ")
    (write-authority-list out "promotion_requires" requires) (format out ", ")
    (format out "\"promotion_owner\": \"~A\", " owner)
    (write-authority-list out "evidence" evidence) (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-cold-start-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-cold-start-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationcoldstartgen.sh check\"}")))
  (format out "~%  ]"))
