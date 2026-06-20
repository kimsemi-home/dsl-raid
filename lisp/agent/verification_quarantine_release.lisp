(in-package #:dslraid.agent)

(defparameter *verification-quarantine-release-required*
  '("independent-replay" "tool-integrity" "evidence-untampered"
    "prompt-injection-contained" "permission-boundary-maintained"
    "reviewer-isolation-restored" "evidence-quality-reassessed"
    "confidence-assessor-approved"))

(defparameter *verification-quarantine-release-gates*
  '(("quarantine-release:confirmed-clean" "containment:quarantine"
     "steward:ops" "confirmed-clean"
     ("independent-replay" "tool-integrity" "evidence-untampered"
      "prompt-injection-contained" "permission-boundary-maintained"
      "reviewer-isolation-restored" "evidence-quality-reassessed"
      "confidence-assessor-approved")
     ("raw-evidence" "claim" "artifact") nil nil
     ("docs/generated/verification-quarantine.json"
      "docs/generated/verification-evidence-quality.json"
      "docs/generated/verification-confidence.json")
     nil "Clean quarantine releases only after every condition is met.")
    ("quarantine-release:partial-reuse" "containment:prompt-injection"
     "steward:security" "partial-release"
     ("independent-replay" "tool-integrity" "evidence-untampered"
      "permission-boundary-maintained")
     ("raw-evidence") ("claim") ("artifact")
     ("docs/generated/verification-quarantine.json"
      "docs/generated/verification-security-audit.json")
     nil "Partial release may reuse raw evidence while invalidating claims.")
    ("quarantine-release:aged-debt" "containment:stale-review"
     "steward:ops" "debt-open"
     ("evidence-quality-reassessed") nil nil nil
     ("docs/generated/verification-quarantine.json"
      "docs/generated/verification-debt.json")
     ("debt:quarantine-aging") "Stale quarantine becomes visible debt.")))

(defparameter *verification-quarantine-release-rules*
  '(("quarantine-release:steward" "Only a steward releases quarantine.")
    ("quarantine-release:conditions" "Confirmed clean requires all conditions.")
    ("quarantine-release:partial" "Partial release declares reuse and invalidation.")
    ("quarantine-release:debt" "Aged quarantine creates tracked debt.")))

(defun emit-verification-quarantine-release-json (&optional stream)
  "Emit quarantine release gates."
  (let ((json (with-output-to-string (out)
                (write-verification-quarantine-release out))))
    (if stream (write-string json stream) json)))

(defun write-verification-quarantine-release (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationquarantinereleasegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_quarantine_release.lisp\",~%")
  (format out "  \"quarantine_release_profile\": \"conditioned-release\",~%")
  (write-quarantine-release-required out)
  (format out ",~%")
  (write-quarantine-release-gates out)
  (format out ",~%")
  (write-quarantine-release-rules out)
  (format out "~%}~%"))

(defun write-quarantine-release-required (out)
  (format out "  \"required_conditions\": [~%")
  (write-json-items out *verification-quarantine-release-required* 4)
  (format out "  ]"))

(defun write-quarantine-release-gates (out)
  (format out "  \"release_gates\": [~%")
  (loop for row in *verification-quarantine-release-gates*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-quarantine-release-gate out row))
  (format out "~%  ]"))
