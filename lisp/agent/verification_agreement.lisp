(in-package #:dslraid.agent)

(defparameter *verification-agreements*
  '(("agreement:release-check" "agent:verification-daemon" ("reviewer:quality" "reviewer:red-team") "high" "agree" "sealed" "ontology:verification.0.1.0" t ("docs/generated/verification-sidecar.json" "docs/generated/verification-reliability.json") "Release checks need isolated quality and adversarial reviewers.")
    ("agreement:conformance" "agent:quality-runner" ("reviewer:conformance" "reviewer:quality") "routine" "agree" "sealed" "ontology:verification.0.1.0" nil ("docs/generated/verification-conformance.json" "docs/generated/verification-evidence-quality.json") "Conformance agreement needs independent reviewers and generated evidence.")
    ("agreement:security" "agent:verification-daemon" ("reviewer:security" "reviewer:red-team") "audit" "blocked" "sealed" "ontology:verification.0.1.0" t ("docs/generated/verification-security-audit.json" "docs/generated/verification-authority.json") "Audit-risk agreement stays blocked until security authority closes.")))

(defparameter *verification-agreement-rules*
  '(("agreement:independent-reviewer" "Agreement requires reviewers independent from producer.")
    ("agreement:adversarial-required" "High and audit risk require adversarial review.")
    ("agreement:evidence-required" "Agreement cites generated evidence.")))

(defun emit-verification-agreement-json (&optional stream)
  "Emit cross-agent agreement receipts."
  (let ((json (with-output-to-string (out) (write-verification-agreement out))))
    (if stream (write-string json stream) json)))

(defun write-verification-agreement (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationagreementgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_agreement.lisp\",~%")
  (format out "  \"agreement_profile\": \"isolated-review\",~%")
  (write-agreements out)
  (format out ",~%")
  (write-agreement-rules out)
  (format out "~%}~%"))

(defun write-agreements (out)
  (format out "  \"agreements\": [~%")
  (loop for row in *verification-agreements*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-agreement out row))
  (format out "~%  ]"))

(defun write-agreement (out row)
  (destructuring-bind (id producer reviewers risk decision isolation ontology adversarial evidence meaning) row
    (format out "    {\"id\": \"~A\", \"producer\": \"~A\", " id producer)
    (write-authority-list out "reviewers" reviewers)
    (format out ", \"risk\": \"~A\", \"decision\": \"~A\", " risk decision)
    (format out "\"isolation\": \"~A\", \"ontology\": \"~A\", " isolation ontology)
    (format out "\"adversarial\": ~:[false~;true~], " adversarial)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-agreement-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-agreement-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationagreementgen.sh check\"}")))
  (format out "~%  ]"))
