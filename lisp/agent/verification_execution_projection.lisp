(in-package #:dslraid.agent)

(defparameter *execution-projection-backends*
  '(("execution-projection:github-actions" "github-actions"
     ".github/workflows/verification.yml" "scripts/workflowgen.sh"
     "GitHub Actions must carry representative graph commands.")
    ("execution-projection:gitlab-ci" "gitlab-ci" ".gitlab-ci.yml"
     "scripts/gitlabgen.sh" "GitLab CI must carry representative graph commands.")
    ("execution-projection:local-makefile" "local-makefile" "Makefile"
     "scripts/makegen.sh" "Makefile must carry representative graph commands.")
    ("execution-projection:bazel" "bazel" "BUILD.bazel"
     "scripts/bazelgen.sh" "Bazel must carry representative graph commands.")))

(defparameter *execution-projection-command-probes*
  '("bash scripts/check-source-lines.sh"
    "cargo test --workspace"
    "npm --prefix apps/viewer run build"
    "cargo run -p dslraid-cli -- quality"
    "bash scripts/releasecheck/artifacts.sh"))

(defparameter *execution-projection-rules*
  '(("execution-projection:commands"
     "Every execution backend must contain representative graph commands.")
    ("execution-projection:nodes"
     "Every execution backend must expose the graph node set.")
    ("execution-projection:generators"
     "Every execution backend must remain generated and checkable.")))

(defun emit-verification-execution-projection-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-execution-projection out))))
    (if stream (write-string json stream) json)))

(defun write-verification-execution-projection (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationprojectiongen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_execution_projection.lisp\",~%")
  (format out "  \"execution_projection_profile\": \"same-command-projection\",~%")
  (write-execution-projections out)
  (format out ",~%")
  (write-execution-projection-rules out)
  (format out "~%}~%"))

(defun write-execution-projections (out)
  (format out "  \"projections\": [~%")
  (loop for row in *execution-projection-backends* for first = t then nil
        do (unless first (format out ",~%"))
           (write-execution-projection out row))
  (format out "~%  ]"))

(defun write-execution-projection (out row)
  (destructuring-bind (id backend output generator meaning) row
    (format out "    {\"id\": \"~A\", \"backend\": \"~A\", " id backend)
    (format out "\"output\": \"~A\", \"generator\": \"~A\", " output generator)
    (write-authority-list out "graph_nodes" (mapcar #'verification-id (verification-nodes)))
    (format out ", ")
    (write-authority-list out "command_probes" *execution-projection-command-probes*)
    (format out ", \"check\": \"~A check\", " generator)
    (format out "\"meaning\": \"~A\"}" meaning)))

(defun write-execution-projection-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *execution-projection-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationprojectiongen.sh check\"}")))
  (format out "~%  ]"))
