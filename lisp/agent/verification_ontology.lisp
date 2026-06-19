(in-package #:dslraid.agent)

(defparameter *verification-ontology-chain*
  '(("ontology" "Names the meaning model for generated verification surfaces.")
    ("executable-ssot" "Keeps the verification graph in Lisp as source.")
    ("verification-graph" "Defines ordered gates and evidence commands.")
    ("codegen" "Derives deterministic artifacts and checks.")))

(defparameter *verification-codegen-axes*
  '("code" "docs" "schemas" "tests" "conformance"
    "github-actions" "release-pipelines" "pdca-evidence" "loss-ledger"
    "semantic-hash" "semantic-diff" "authority-gate" "evidence-quality"
    "lease-and-abort" "review-capacity" "feedback-closure" "quarantine"))

(defun emit-verification-ontology-json (&optional stream)
  "Emit machine-readable ontology chain for the verification graph."
  (let ((json (with-output-to-string (out)
                (write-verification-ontology out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-verification-ontology (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationontologygen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"ssot\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"form\": \"~A\",~%" (getf (verification-graph) :form))
  (write-ontology-chain out)
  (format out ",~%")
  (write-codegen-axes out)
  (format out "~%}~%"))

(defun write-ontology-chain (out)
  (format out "  \"chain\": [~%")
  (loop for row in *verification-ontology-chain*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\"}" id meaning)))
  (format out "~%  ]"))

(defun write-codegen-axes (out)
  (format out "  \"codegen_axes\": [~%")
  (write-json-items out *verification-codegen-axes* 4)
  (format out "  ]"))
