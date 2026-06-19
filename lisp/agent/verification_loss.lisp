(in-package #:dslraid.agent)

(defparameter *verification-loss-ledger*
  '(("loss:github-actions" "verify:daemon" ".github/workflows/verification.yml"
     "L2" "Runner YAML keeps command order, not full ontology semantics."
     "docs/generated/verification-evidence.json")
    ("loss:gitlab-ci" "verify:daemon" ".gitlab-ci.yml"
     "L2" "GitLab syntax preserves gates but changes runner semantics."
     "docs/generated/verification-evidence.json")
    ("loss:local-makefile" "verify:daemon" "Makefile"
     "L1" "Make exposes local targets without hosted runner metadata."
     "docs/generated/verification-conformance.json")
    ("loss:manifest-schema" "verify:daemon"
     "schemas/dslraid-verification-manifest.schema.json"
     "L1" "Schema contract preserves manifest structure, not Lisp form syntax."
     "docs/generated/verification-conformance.json")
    ("loss:bazel" "verify:daemon" "BUILD.bazel"
     "L1" "Bazel genrules keep command chains without workflow UI labels."
     "docs/generated/verification-conformance.json")))

(defparameter *verification-loss-rules*
  '(("loss:no-l4-loss" "Forbidden L4 loss is not allowed in generated adapters.")
    ("loss:evidence-linked" "Every loss entry links to generated evidence.")
    ("loss:public-safe" "Loss ledger contains public paths only.")))

(defun emit-verification-loss-json (&optional stream)
  "Emit machine-readable loss ledger for generated verification adapters."
  (let ((json (with-output-to-string (out)
                (write-verification-loss out))))
    (if stream
        (write-string json stream)
        json)))

(defun write-verification-loss (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationlossgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (write-loss-ledger out)
  (format out ",~%")
  (write-loss-rules out)
  (format out "~%}~%"))

(defun write-loss-ledger (out)
  (format out "  \"ledger\": [~%")
  (loop for row in *verification-loss-ledger*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-loss-entry out row))
  (format out "~%  ]"))

(defun write-loss-entry (out row)
  (destructuring-bind (id source target level meaning evidence) row
    (format out "    {\"id\": \"~A\", \"source\": \"~A\", " id source)
    (format out "\"target\": \"~A\", \"loss_level\": \"~A\", " target level)
    (format out "\"meaning\": \"~A\", \"evidence\": \"~A\", " meaning evidence)
    (format out "\"policy\": \"Record adapter loss before release.\"}")))

(defun write-loss-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-loss-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationlossgen.sh check\"}")))
  (format out "~%  ]"))
