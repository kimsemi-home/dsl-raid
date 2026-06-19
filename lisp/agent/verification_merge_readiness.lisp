(in-package #:dslraid.agent)

(defparameter *verification-merge-readiness-gates*
  '(("merge-ready:quality" "quality"
     "cargo run -p dslraid-cli -- quality"
     ("docs/generated/verification-conformance.json")
     "Unified quality must pass before merge.")
    ("merge-ready:golden" "golden"
     "cargo run -p dslraid-cli -- golden check tests/golden"
     ("tests/golden/verification-graph.generated.json")
     "Golden evidence must match generated outputs before merge.")
    ("merge-ready:privacy" "privacy"
     "bash scripts/privacycheck.sh check"
     ("docs/generated/verification-privacy.json")
     "Public surfaces must exclude private paths and credential values.")
    ("merge-ready:line-budget" "line-budget"
     "bash scripts/check-source-lines.sh"
     ("docs/generated/verification-codegen.json")
     "Small-file architecture must stay within the line budget.")
    ("merge-ready:pages" "pages"
     "bash scripts/workflowgen.sh check"
     (".github/workflows/pages.yml"
      "docs/generated/verification-github-actions.json")
     "Published demo must be backed by generated workflow evidence.")))

(defparameter *verification-merge-readiness-rules*
  '(("merge-ready:all-gates-pass" "Every merge gate must have a check command.")
    ("merge-ready:evidence-present" "Every gate must cite file-backed evidence.")
    ("merge-ready:privacy-required" "Privacy is a merge gate, not a post-check.")
    ("merge-ready:generated-workflows-fresh" "Generated workflows must be fresh.")))

(defun emit-verification-merge-readiness-json (&optional stream)
  "Emit merge readiness gates for autonomous merge decisions."
  (let ((json (with-output-to-string (out)
                (write-verification-merge-readiness out))))
    (if stream (write-string json stream) json)))

(defun write-verification-merge-readiness (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationmergegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_merge_readiness.lisp\",~%")
  (format out "  \"merge_profile\": \"autonomous-merge-gated\",~%")
  (write-merge-readiness-gates out)
  (format out ",~%")
  (write-merge-readiness-rules out)
  (format out "~%}~%"))

(defun write-merge-readiness-gates (out)
  (format out "  \"gates\": [~%")
  (loop for row in *verification-merge-readiness-gates*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-merge-readiness-gate out row))
  (format out "~%  ]"))

(defun write-merge-readiness-gate (out row)
  (destructuring-bind (id gate check evidence meaning) row
    (format out "    {\"id\": \"~A\", \"gate\": \"~A\", " id gate)
    (format out "\"check\": \"~A\", " check)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"required\", \"meaning\": \"~A\"}" meaning)))

(defun write-merge-readiness-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-merge-readiness-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationmergegen.sh check\"}")))
  (format out "~%  ]"))
