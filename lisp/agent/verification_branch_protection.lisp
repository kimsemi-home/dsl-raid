(in-package #:dslraid.agent)

(defparameter *verification-branch-protection*
  '(("branch-protect:main" "branch" "main"
     ".github/workflows/ci.yml" ("docs/generated/verification-merge-automation.json")
     "Main branch protection is the target merge boundary.")
    ("branch-protect:ci" "required-check" "CI"
     ".github/workflows/ci.yml" ("docs/generated/verification-conformance.json")
     "CI must stay required before protected merges.")
    ("branch-protect:security" "required-check" "Security"
     ".github/workflows/security.yml" ("docs/generated/verification-security-audit.json")
     "Security checks must stay required before protected merges.")
    ("branch-protect:golden" "required-check" "Golden"
     ".github/workflows/golden.yml" ("tests/golden/verification-graph.generated.json")
     "Golden output checks must stay required before protected merges.")
    ("branch-protect:verification" "required-check" "Verification Graph"
     ".github/workflows/verification.yml" ("docs/generated/verification-backend-parity.json")
     "Generated Verification Graph must stay required before protected merges.")))

(defparameter *verification-branch-protection-rules*
  '(("branch-protect:main-only" "Protected merge target must be main.")
    ("branch-protect:required-checks" "Required check workflows must exist.")
    ("branch-protect:no-target-event" "Required PR checks must avoid pull_request_target.")
    ("branch-protect:evidence-linked" "Each required check must cite evidence.")))

(defun emit-verification-branch-protection-json (&optional stream)
  "Emit desired branch protection evidence for protected merges."
  (let ((json (with-output-to-string (out)
                (write-verification-branch-protection out))))
    (if stream (write-string json stream) json)))

(defun write-verification-branch-protection (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationbranchgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_branch_protection.lisp\",~%")
  (format out "  \"branch_protection_profile\": \"main-required-checks\",~%")
  (write-branch-protection-requirements out)
  (format out ",~%")
  (write-branch-protection-rules out)
  (format out "~%}~%"))

(defun write-branch-protection-requirements (out)
  (format out "  \"requirements\": [~%")
  (loop for row in *verification-branch-protection*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-branch-protection-requirement out row))
  (format out "~%  ]"))

(defun write-branch-protection-requirement (out row)
  (destructuring-bind (id kind name workflow evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"name\": \"~A\", \"workflow\": \"~A\", " name workflow)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"required\", \"meaning\": \"~A\"}" meaning)))

(defun write-branch-protection-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-branch-protection-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationbranchgen.sh check\"}")))
  (format out "~%  ]"))
