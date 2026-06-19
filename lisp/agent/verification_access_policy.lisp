(in-package #:dslraid.agent)

(defparameter *verification-access-policies*
  '(("access:producer-public" "producer" "public-surface" "allow" "rbac:producer" "abac:public" ("docs/generated/verification-privacy.json" "docs/generated/verification-evidence.json") "steward:release" "Producers may update public surfaces only after generated privacy evidence.")
    ("access:producer-private" "producer" "private-data" "deny" "rbac:producer" "abac:private" ("docs/generated/verification-privacy.json" "docs/generated/verification-security-audit.json") "governance:privacy" "Private data never enters public generated artifacts.")
    ("access:verifier-security" "verifier" "security-boundary" "escalate" "rbac:verifier" "abac:security" ("docs/generated/verification-security-audit.json" "docs/generated/verification-authority.json") "governance:security" "Security boundary checks escalate to governance authority.")
    ("access:steward-release" "steward" "release" "allow" "rbac:steward" "abac:release" ("docs/generated/verification-conformance.json" "docs/generated/verification-debt.json") "steward:release" "Release authority needs conformance and visible debt.")
    ("access:governance-ontology" "governance" "ontology" "allow" "rbac:governance" "abac:ontology" ("docs/generated/verification-ontology-transition.json" "docs/generated/verification-historical-interpreter.json") "governance:ontology" "Ontology changes require governance and transition evidence.")))

(defparameter *verification-access-rules*
  '(("access:rbac-required" "Every policy has an explicit role gate.")
    ("access:abac-required" "Every policy has a context gate.")
    ("access:private-denied" "Private data policy cannot allow public authority.")))

(defun emit-verification-access-policy-json (&optional stream)
  "Emit RBAC and ABAC access policy gates."
  (let ((json (with-output-to-string (out) (write-verification-access-policy out))))
    (if stream (write-string json stream) json)))

(defun write-verification-access-policy (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationaccessgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_access_policy.lisp\",~%")
  (format out "  \"access_profile\": \"reasoning-rbac-abac\",~%")
  (write-access-policies out)
  (format out ",~%")
  (write-access-rules out)
  (format out "~%}~%"))

(defun write-access-policies (out)
  (format out "  \"policies\": [~%")
  (loop for row in *verification-access-policies*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-access-policy out row))
  (format out "~%  ]"))

(defun write-access-policy (out row)
  (destructuring-bind (id role context decision rbac abac evidence authority meaning) row
    (format out "    {\"id\": \"~A\", \"role\": \"~A\", " id role)
    (format out "\"context\": \"~A\", \"decision\": \"~A\", " context decision)
    (format out "\"rbac\": \"~A\", \"abac\": \"~A\", " rbac abac)
    (write-authority-list out "evidence" evidence)
    (format out ", \"authority\": \"~A\", " authority)
    (format out "\"meaning\": \"~A\"}" meaning)))

(defun write-access-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-access-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationaccessgen.sh check\"}")))
  (format out "~%  ]"))
