(in-package #:dslraid.agent)

(defparameter *verification-security-boundaries*
  '(("security-audit:permission-boundary" "permission-change"
     "policy:verification-permission-change"
     "docs/generated/verification-semantic-hash.json"
     ("authz:verification-release" "authz:generated-artifact")
     "rbac:verification-gate" "abac:public-generated-evidence"
     "audit:permission-change" "docs/generated/verification-conformance.json"
     "human:security-steward" "rollback:restore-prior-policy" "gate:security"
     "high"
     ("docs/generated/verification-authority.json"
      "docs/generated/verification-semantic-hash.json"
      "docs/generated/verification-conformance.json")
     "Permission changes require policy, audit, approval, and rollback.")))

(defparameter *verification-security-rules*
  '(("security:policy-first" "Permission changes start from policy DSL.")
    ("security:audit-required" "Audit and conformance evidence are required.")
    ("security:human-approval" "High risk permission changes require human approval.")))

(defun emit-verification-security-audit-json (&optional stream)
  "Emit security and audit boundary receipts."
  (let ((json (with-output-to-string (out)
                (write-verification-security-audit out))))
    (if stream (write-string json stream) json)))

(defun write-verification-security-audit (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationsecuritygen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"security_audit_profile\": \"permission-boundary\",~%")
  (write-security-boundaries out)
  (format out ",~%")
  (write-security-rules out)
  (format out "~%}~%"))

(defun write-security-boundaries (out)
  (format out "  \"boundaries\": [~%")
  (loop for row in *verification-security-boundaries*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-security-boundary out row))
  (format out "~%  ]"))

(defun write-security-boundary (out row)
  (destructuring-bind (id kind policy hash paths rbac abac audit conformance approval rollback authority risk evidence meaning) row
    (format out "    {\"id\": \"~A\", \"change_kind\": \"~A\", " id kind)
    (format out "\"policy\": \"~A\", \"semantic_hash\": \"~A\", " policy hash)
    (write-authority-list out "affected_paths" paths)
    (format out ", \"rbac_check\": \"~A\", \"abac_check\": \"~A\", " rbac abac)
    (format out "\"audit_check\": \"~A\", \"conformance\": \"~A\", " audit conformance)
    (format out "\"approval\": \"~A\", \"rollback\": \"~A\", " approval rollback)
    (format out "\"authority\": \"~A\", \"risk\": \"~A\", " authority risk)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-security-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-security-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationsecuritygen.sh check\"}")))
  (format out "~%  ]"))
