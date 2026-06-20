(in-package #:dslraid.agent)

(defparameter *workflow-lineage-nodes*
  '("lint" "unit-test" "integration-test" "conformance" "release-check"))

(defparameter *verification-workflow-lineages*
  `(("workflow-lineage:github-actions" "github-actions"
     ,*workflow-lineage-nodes* ".github/workflows/verification.yml"
     "scripts/workflowgen.sh" "scripts/workflowgen.sh check"
     ("docs/generated/verification-github-actions.json"
      "docs/generated/verification-backend-parity.json")
     "GitHub Actions runs the generated Verification Graph.")
    ("workflow-lineage:gitlab-ci" "gitlab-ci"
     ,*workflow-lineage-nodes* ".gitlab-ci.yml"
     "scripts/gitlabgen.sh" "scripts/gitlabgen.sh check"
     ("docs/generated/verification-backend-parity.json")
     "GitLab CI is a projection of the same graph.")
    ("workflow-lineage:makefile" "local-makefile"
     ,*workflow-lineage-nodes* "Makefile"
     "scripts/makegen.sh" "scripts/makegen.sh check"
     ("docs/generated/verification-execution-projection.json")
     "Local Makefile keeps the graph runnable without CI.")
    ("workflow-lineage:bazel" "bazel"
     ,*workflow-lineage-nodes* "BUILD.bazel"
     "scripts/bazelgen.sh" "scripts/bazelgen.sh check"
     ("docs/generated/verification-execution-projection.json")
     "Bazel receives the same ordered verification graph.")
    ("workflow-lineage:release-check" "release-check-provider"
     ("release-check") "scripts/releasecheck"
     "scripts/releasecheckgen.sh" "scripts/releasecheckgen.sh check"
     ("docs/generated/verification-quality-closure.json"
      "docs/generated/verification-failure-recovery.json")
     "Release-check providers expand the release-check node.")))

(defparameter *verification-workflow-lineage-rules*
  '(("workflow-lineage:nodes-exist" "Lineage nodes exist in the Verification Graph.")
    ("workflow-lineage:artifact-generated" "Lineage artifact has a generator.")
    ("workflow-lineage:evidence-linked" "Lineage cites generated evidence.")))

(defun emit-verification-workflow-lineage-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-workflow-lineage out))))
    (if stream (write-string json stream) json)))

(defun write-verification-workflow-lineage (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationlineagegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_workflow_lineage.lisp\",~%")
  (format out "  \"workflow_lineage_profile\": \"graph-to-execution-surfaces\",~%")
  (write-workflow-lineages out) (format out ",~%")
  (write-workflow-lineage-rules out) (format out "~%}~%"))

(defun write-workflow-lineages (out)
  (format out "  \"lineages\": [~%")
  (loop for row in *verification-workflow-lineages* for first = t then nil
        do (unless first (format out ",~%")) (write-workflow-lineage out row))
  (format out "~%  ]"))

(defun write-workflow-lineage (out row)
  (destructuring-bind (id surface nodes artifact generator check evidence meaning) row
    (format out "    {\"id\": \"~A\", \"surface\": \"~A\", " id surface)
    (write-authority-list out "graph_nodes" nodes) (format out ", ")
    (format out "\"artifact\": \"~A\", \"generator\": \"~A\", " artifact generator)
    (format out "\"check\": \"~A\", " check) (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"generated\", \"meaning\": \"~A\"}" meaning)))

(defun write-workflow-lineage-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-workflow-lineage-rules* for first = t then nil
        do (unless first (format out ",~%")) (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationlineagegen.sh check\"}")))
  (format out "~%  ]"))
