(in-package #:dslraid.agent)

(defparameter *verification-privacy-rules*
  '(("privacy:no-user-home-paths"
     "Public surfaces must not expose local user home paths."
     "scripts/privacycheck.sh check")
    ("privacy:no-token-like-strings"
     "Public surfaces must not expose token-like secrets."
     "scripts/privacycheck.sh check")
    ("privacy:generated-artifacts-public"
     "Generated verification artifacts must pass the public-safe gate."
     "scripts/verificationprivacygen.sh check")))

(defparameter *verification-public-roots*
  '("docs" "examples" "schemas" "tests/golden" ".github/workflows"
    "scripts" "lisp" "crates" "apps/viewer/src" "apps/viewer/tests"))

(defun emit-verification-privacy-json (&optional stream)
  "Emit machine-readable privacy evidence for generated verification."
  (let ((json (with-output-to-string (out)
                (write-verification-privacy out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-verification-privacy (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationprivacygen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"visibility\": \"public\",~%")
  (write-privacy-roots out)
  (format out ",~%")
  (write-privacy-rules out)
  (format out "~%}~%"))

(defun write-privacy-roots (out)
  (format out "  \"public_surface_roots\": [~%")
  (write-json-items out *verification-public-roots* 4)
  (format out "  ]"))

(defun write-privacy-rules (out)
  (format out "  \"rules\": [~%")
  (loop for row in *verification-privacy-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-privacy-rule out row))
  (format out "~%  ]"))

(defun write-privacy-rule (out row)
  (destructuring-bind (id meaning check) row
    (format out "    {\"id\": \"~A\", " id)
    (format out "\"meaning\": \"~A\", " meaning)
    (format out "\"check\": \"~A\", " check)
    (format out "\"status\": \"required\"}")))
