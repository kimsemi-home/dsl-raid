(in-package #:dslraid.agent)

(defparameter *verification-context-map-translations*
  '(("context-map:lisp-to-github-actions" "context:verification"
     "context:github-actions" "0.1.0" "0.1.0" "adapter:workflowgen"
     "loss:github-actions"
     ("docs/generated/verification-loss-ledger.json"
      ".github/workflows/verification.yml")
     "Lisp verification graph is translated into GitHub Actions gates.")
    ("context-map:lisp-to-manifest-schema" "context:verification"
     "context:manifest-contract" "0.1.0" "0.1.0"
     "adapter:manifest-schema" "loss:manifest-schema"
     ("docs/generated/verification-versioned-ssot.json"
      "schemas/dslraid-verification-manifest.schema.json")
     "Verification graph is translated into a manifest schema contract.")))

(defparameter *verification-context-map-rules*
  '(("context-map:explicit-contexts" "Cross-context movement names both contexts.")
    ("context-map:versioned-bridge" "Translation bridge versions are explicit.")
    ("context-map:loss-linked" "Lossy adapters link a loss ledger entry.")))

(defun emit-verification-context-map-json (&optional stream)
  "Emit context translation bridge receipts for verification artifacts."
  (let ((json (with-output-to-string (out)
                (write-verification-context-map out))))
    (if stream (write-string json stream) json)))

(defun write-verification-context-map (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationcontextmapgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"context_map_profile\": \"versioned-translation-bridge\",~%")
  (write-context-map-translations out)
  (format out ",~%")
  (write-context-map-rules out)
  (format out "~%}~%"))

(defun write-context-map-translations (out)
  (format out "  \"translations\": [~%")
  (loop for row in *verification-context-map-translations*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-context-map-translation out row))
  (format out "~%  ]"))

(defun write-context-map-translation (out row)
  (destructuring-bind (id from to from-version to-version adapter loss evidence meaning) row
    (format out "    {\"id\": \"~A\", \"source_context\": \"~A\", " id from)
    (format out "\"target_context\": \"~A\", " to)
    (format out "\"source_version\": \"~A\", \"target_version\": \"~A\", " from-version to-version)
    (format out "\"adapter\": \"~A\", \"loss_policy\": \"~A\", " adapter loss)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-context-map-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-context-map-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationcontextmapgen.sh check\"}")))
  (format out "~%  ]"))
