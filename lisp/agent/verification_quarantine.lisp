(in-package #:dslraid.agent)

(defparameter *verification-quarantine-bundles*
  '(("quarantine:suspicious-generated-output" "generated-artifact"
     "isolated" "gate:quarantine"
     ("artifact-commit" "confidence-increase" "automatic-approval")
     ("docs/generated/verification-authority.json"
      "docs/generated/verification-semantic-diff.json")
     "Suspicious generated output is isolated before promotion.")
    ("quarantine:tool-behavior" "tool-execution"
     "isolated" "gate:quarantine"
     ("artifact-commit" "confidence-increase" "automatic-approval")
     ("docs/generated/verification-review-capacity.json"
      "docs/generated/verification-feedback.json")
     "Suspicious tool behavior requires review before authority applies.")))

(defparameter *verification-quarantine-rules*
  '(("quarantine:evidence-linked" "Every quarantine bundle links generated evidence.")
    ("quarantine:blocks-commit" "Quarantine blocks artifact commit.")
    ("quarantine:blocks-confidence" "Quarantine blocks confidence increase.")
    ("quarantine:blocks-approval" "Quarantine blocks automatic approval.")))

(defun emit-verification-quarantine-json (&optional stream)
  "Emit quarantine sidecar for suspicious verification artifacts."
  (let ((json (with-output-to-string (out)
                (write-verification-quarantine out))))
    (if stream (write-string json stream) json)))

(defun write-verification-quarantine (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationquarantinegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"quarantine_profile\": \"promotion-blocking-isolation\",~%")
  (write-quarantine-bundles out)
  (format out ",~%")
  (write-quarantine-rules out)
  (format out "~%}~%"))

(defun write-quarantine-bundles (out)
  (format out "  \"bundles\": [~%")
  (loop for row in *verification-quarantine-bundles*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-quarantine-bundle out row))
  (format out "~%  ]"))

(defun write-quarantine-bundle (out row)
  (destructuring-bind (id subject status owner blocks evidence meaning) row
    (format out "    {\"id\": \"~A\", \"subject_kind\": \"~A\", " id subject)
    (format out "\"status\": \"~A\", \"owner\": \"~A\", " status owner)
    (write-authority-list out "blocks" blocks)
    (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))
