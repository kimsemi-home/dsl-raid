(in-package #:dslraid.agent)

(defparameter *verification-translation-checks*
  '(("translation-verifier:github-actions" "context-map:lisp-to-github-actions"
     "context:verification" "context:github-actions" "loss:github-actions" "L2"
     "verified"
     ("docs/generated/verification-context-map.json"
      "docs/generated/verification-loss-ledger.json")
     "GitHub Actions translation is verified with a recorded non-forbidden loss.")
    ("translation-verifier:manifest-schema" "context-map:lisp-to-manifest-schema"
     "context:verification" "context:manifest-contract" "loss:manifest-schema" "L1"
     "verified"
     ("docs/generated/verification-context-map.json"
      "docs/generated/verification-loss-ledger.json")
     "Manifest schema translation is verified with recorded structural loss.")))

(defparameter *verification-translation-rules*
  '(("translation-verifier:coverage" "Every context-map translation has a verifier check.")
    ("translation-verifier:loss-known" "Every loss policy resolves to the loss ledger.")
    ("translation-verifier:no-forbidden-loss" "Verified translations cannot carry L4 loss.")))

(defun emit-verification-translation-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-translation out))))
    (if stream (write-string json stream) json)))

(defun write-verification-translation (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationtranslationgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_translation_verifier.lisp\",~%")
  (format out "  \"translation_verifier_profile\": \"loss-ledger-sidecar\",~%")
  (write-translation-checks out)
  (format out ",~%")
  (write-translation-rules out)
  (format out "~%}~%"))

(defun write-translation-checks (out)
  (format out "  \"checks\": [~%")
  (loop for row in *verification-translation-checks* for first = t then nil
        do (unless first (format out ",~%")) (write-translation-check out row))
  (format out "~%  ]"))

(defun write-translation-check (out row)
  (destructuring-bind (id translation from to loss level verdict evidence meaning) row
    (format out "    {\"id\": \"~A\", \"translation\": \"~A\", " id translation)
    (format out "\"source_context\": \"~A\", \"target_context\": \"~A\", " from to)
    (format out "\"loss_policy\": \"~A\", \"loss_level\": \"~A\", " loss level)
    (format out "\"verdict\": \"~A\", " verdict)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-translation-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-translation-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationtranslationgen.sh check\"}")))
  (format out "~%  ]"))
