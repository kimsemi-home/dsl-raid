(in-package #:dslraid.agent)

(defparameter *verification-pdca-steps*
  '(("plan" "Lisp SSOT names the intended verification gates."
     "lisp/agent/verification.lisp")
    ("do" "Generators derive CI, docs, schemas, tests, and manifests."
     "docs/generated/verification-evidence.json")
    ("check" "Quality and release-check rerun deterministic generated checks."
     "docs/generated/verification-conformance.json")
    ("act" "Stale output fails until SSOT or generated artifacts are updated."
     "tests/golden/verification-graph.generated.json")))

(defparameter *verification-pdca-rules*
  '(("pdca:loop-has-four-phases" "plan, do, check, and act are present.")
    ("pdca:generated-output-is-checked" "Every generated backend has a check.")
    ("pdca:stale-output-is-actionable" "Diff failure names the generator.")))

(defun emit-verification-pdca-json (&optional stream)
  "Emit machine-readable PDCA loop evidence for generated verification."
  (let ((json (with-output-to-string (out)
                (write-verification-pdca out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-verification-pdca (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationpdcagen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (write-pdca-manifest-loop out)
  (format out ",~%")
  (write-pdca-rules out)
  (format out "~%}~%"))

(defun write-pdca-manifest-loop (out)
  (format out "  \"loop\": [~%")
  (loop for row in *verification-pdca-steps*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-pdca-step out row))
  (format out "~%  ]"))

(defun write-pdca-step (out row)
  (destructuring-bind (phase evidence artifact) row
    (format out "    {\"phase\": \"~A\", " phase)
    (format out "\"evidence\": \"~A\", " evidence)
    (format out "\"artifact\": \"~A\"}" artifact)))

(defun write-pdca-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-pdca-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-pdca-rule out row))
  (format out "~%  ]"))

(defun write-pdca-rule (out row)
  (destructuring-bind (id meaning) row
    (format out "    {\"id\": \"~A\", " id)
    (format out "\"meaning\": \"~A\", " meaning)
    (format out "\"check\": \"scripts/verificationpdcagen.sh check\"}")))
