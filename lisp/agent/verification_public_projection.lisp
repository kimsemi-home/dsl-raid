(in-package #:dslraid.agent)

(defparameter *verification-public-projection-decisions*
  '(("public-projection:roots" "include" "public-roots" "public-surface"
     "included" ("docs/generated/verification-privacy.json"
                 "scripts/privacycheck.sh")
     "Only declared public roots may be published.")
    ("public-projection:private-data" "exclude" "private-data"
     "public-artifacts" "excluded"
     ("docs/generated/verification-access-policy.json"
      "docs/generated/verification-security-audit.json")
     "Private data is denied before public artifact generation.")
    ("public-projection:secret-artifacts" "redact" "secret-bearing-artifact"
     "public-projection" "blocked"
     ("docs/generated/assertion-catalog.md" "schemas/dslraid-core.schema.json")
     "Secret artifacts cannot survive a public projection.")
    ("public-projection:local-paths" "redact" "local-user-path"
     "public-surface" "blocked"
     ("scripts/privacycheck.sh" "docs/generated/verification-privacy.json")
     "Local user paths are blocked from public generated evidence.")))

(defparameter *verification-public-projection-rules*
  '(("public-projection:evidence-linked"
     "Every projection decision cites executable evidence.")
    ("public-projection:no-private-allow"
     "Private and secret sources cannot be allowed.")
    ("public-projection:privacy-check"
     "The public surface must pass privacycheck.")))

(defun emit-verification-public-projection-json (&optional stream)
  "Emit private-to-public projection boundary evidence."
  (let ((json (with-output-to-string (out)
                (write-verification-public-projection out))))
    (if stream (write-string json stream) json)))

(defun write-verification-public-projection (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationpublicprojectiongen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_public_projection.lisp\",~%")
  (format out "  \"public_projection_profile\": \"private-to-public-boundary\",~%")
  (write-public-projection-decisions out)
  (format out ",~%")
  (write-public-projection-rules out)
  (format out "~%}~%"))

(defun write-public-projection-decisions (out)
  (format out "  \"decisions\": [~%")
  (loop for row in *verification-public-projection-decisions*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-public-projection-decision out row))
  (format out "~%  ]"))

(defun write-public-projection-decision (out row)
  (destructuring-bind (id kind source target effect evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"source\": \"~A\", \"target\": \"~A\", " source target)
    (format out "\"effect\": \"~A\", " effect)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-public-projection-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-public-projection-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationpublicprojectiongen.sh check\"}")))
  (format out "~%  ]"))
