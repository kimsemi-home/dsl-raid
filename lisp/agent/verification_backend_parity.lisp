(in-package #:dslraid.agent)

(defparameter *verification-parity-backends*
  '(("backend-parity:github-actions" "github-actions"
     ".github/workflows/verification.yml" "scripts/workflowgen.sh"
     "workflow-job-chain" "GitHub Actions projects every graph node.")
    ("backend-parity:gitlab-ci" "gitlab-ci" ".gitlab-ci.yml"
     "scripts/gitlabgen.sh" "stage-chain"
     "GitLab CI projects every graph node with runner-specific syntax.")
    ("backend-parity:local-makefile" "local-makefile" "Makefile"
     "scripts/makegen.sh" "make-target-chain"
     "Makefile projects every graph node for local verification.")
    ("backend-parity:bazel" "bazel" "BUILD.bazel" "scripts/bazelgen.sh"
     "genrule-chain" "Bazel projects every graph node through genrules.")))

(defparameter *verification-parity-rules*
  '(("backend-parity:same-node-set"
     "Every execution backend must project the same verification node set.")
    ("backend-parity:backend-evidence-exists"
     "Every projected backend must exist in generated evidence.")
    ("backend-parity:check-command-exists"
     "Every projected backend must expose a generated check command.")))

(defun emit-verification-backend-parity-json (&optional stream)
  "Emit backend parity evidence for generated verification graph surfaces."
  (let ((json (with-output-to-string (out)
                (write-verification-backend-parity out))))
    (if stream (write-string json stream) json)))

(defun write-verification-backend-parity (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationparitygen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"parity_profile\": \"same-graph-projection\",~%")
  (write-parity-projections out)
  (format out ",~%")
  (write-parity-rules out)
  (format out "~%}~%"))

(defun write-parity-projections (out)
  (format out "  \"projections\": [~%")
  (loop for row in *verification-parity-backends*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-parity-projection out row))
  (format out "~%  ]"))

(defun write-parity-projection (out row)
  (destructuring-bind (id backend output generator projection meaning) row
    (format out "    {\"id\": \"~A\", \"backend\": \"~A\", " id backend)
    (format out "\"output\": \"~A\", \"generator\": \"~A\", " output generator)
    (format out "\"projection\": \"~A\", " projection)
    (write-authority-list out "graph_nodes" (mapcar #'verification-id (verification-nodes)))
    (format out ", \"check\": \"~A check\", " generator)
    (format out "\"meaning\": \"~A\"}" meaning)))

(defun write-parity-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-parity-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationparitygen.sh check\"}")))
  (format out "~%  ]"))
