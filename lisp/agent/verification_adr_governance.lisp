(in-package #:dslraid.agent)

(defparameter *verification-adr-governance*
  '(("adr-governance:core-ir-schema" "core-ir-schema" "semantic-contract"
     "adr-required" "governance:architecture"
     "docs/adr/0002-product-scope-and-risk-boundaries.md"
     ("docs/generated/verification-ontology.json"
      "docs/generated/verification-semantic-diff.json")
     "Core IR schema changes require an ADR before implementation.")
    ("adr-governance:composition" "fsm-composition" "semantic-contract"
     "adr-required" "governance:architecture"
     "docs/adr/0002-product-scope-and-risk-boundaries.md"
     ("docs/generated/verification-control-plane.json"
      "docs/generated/verification-provider-compat.json")
     "Composition semantics are public behavior and require ADR review.")
    ("adr-governance:diagnostic-code" "diagnostic-code" "semantic-contract"
     "adr-required" "governance:quality"
     "docs/validation.md"
     ("docs/generated/verification-failure-conditions.json"
      "docs/generated/verification-evidence-quality.json")
     "Diagnostic code or severity changes require governance review.")
    ("adr-governance:viewer-ui" "viewer-ui" "implementation-detail"
     "autonomous-allowed" "steward:implementation"
     "docs/adr/0005-observable-surface-area.md"
     ("docs/generated/verification-sidecar.json"
      "docs/generated/verification-runtime-trace.json")
     "Viewer and renderer internals may move while preserving public meaning.")))

(defparameter *verification-adr-rules*
  '(("adr:semantic-contract-gated" "Semantic contract changes need ADR.")
    ("adr:policy-doc-exists" "Governance rows cite an existing policy document.")
    ("adr:evidence-generated" "Governance rows cite generated evidence.")))

(defun emit-verification-adr-json (&optional stream)
  "Emit ADR governance boundaries for verification work."
  (let ((json (with-output-to-string (out) (write-verification-adr out))))
    (if stream (write-string json stream) json)))

(defun write-verification-adr (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationadrgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_adr_governance.lisp\",~%")
  (format out "  \"adr_profile\": \"semantic-change-boundary\",~%")
  (write-adr-governance-records out)
  (format out ",~%")
  (write-adr-rules out)
  (format out "~%}~%"))

(defun write-adr-governance-records (out)
  (format out "  \"records\": [~%")
  (loop for row in *verification-adr-governance*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-adr-governance-record out row))
  (format out "~%  ]"))

(defun write-adr-governance-record (out row)
  (destructuring-bind (id surface kind decision authority policy evidence meaning) row
    (format out "    {\"id\": \"~A\", \"surface\": \"~A\", " id surface)
    (format out "\"change_kind\": \"~A\", \"decision\": \"~A\", " kind decision)
    (format out "\"authority\": \"~A\", \"policy_doc\": \"~A\", " authority policy)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-adr-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-adr-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationadrgen.sh check\"}")))
  (format out "~%  ]"))
