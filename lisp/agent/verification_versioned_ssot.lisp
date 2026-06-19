(in-package #:dslraid.agent)

(defparameter *verification-versioned-ssot-scopes*
  '(("versioned-ssot:verification-graph" "context:verification"
     "0.1.0" "0.1.0" "lisp/agent/verification.lisp" "gate:authority"
     ("docs/generated/verification-ontology.json"
      "docs/generated/verification-codegen.json")
     "Verification graph authority is scoped by ontology and contract version.")
    ("versioned-ssot:manifest-schema" "context:manifest-contract"
     "0.1.0" "0.1.0" "schemas/dslraid-verification-manifest.schema.json"
     "gate:schema"
     ("docs/generated/verification-conformance.json"
      "docs/generated/verification-evidence-quality.json")
     "Manifest schema authority is scoped by schema contract version.")))

(defparameter *verification-versioned-ssot-rules*
  '(("versioned-ssot:context" "Every SSOT authority has explicit context.")
    ("versioned-ssot:versions" "Ontology and contract versions are explicit.")
    ("versioned-ssot:evidence" "Version scope links generated evidence.")))

(defun emit-verification-versioned-ssot-json (&optional stream)
  "Emit version-scoped SSOT authority receipts."
  (let ((json (with-output-to-string (out)
                (write-verification-versioned-ssot out))))
    (if stream (write-string json stream) json)))

(defun write-verification-versioned-ssot (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationversionedssotgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"versioned_ssot_profile\": \"context-version-scoped-authority\",~%")
  (write-versioned-ssot-scopes out)
  (format out ",~%")
  (write-versioned-ssot-rules out)
  (format out "~%}~%"))

(defun write-versioned-ssot-scopes (out)
  (format out "  \"scopes\": [~%")
  (loop for row in *verification-versioned-ssot-scopes*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-versioned-ssot-scope out row))
  (format out "~%  ]"))

(defun write-versioned-ssot-scope (out row)
  (destructuring-bind (id context ontology contract ssot authority evidence meaning) row
    (format out "    {\"id\": \"~A\", \"context\": \"~A\", " id context)
    (format out "\"ontology_version\": \"~A\", \"contract_version\": \"~A\", " ontology contract)
    (format out "\"ssot\": \"~A\", \"authority\": \"~A\", " ssot authority)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-versioned-ssot-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-versioned-ssot-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationversionedssotgen.sh check\"}")))
  (format out "~%  ]"))
