(in-package #:dslraid.agent)

(defparameter *verification-evidence-graph-nodes*
  '(("evidence-graph:runtime" "observation" "docs/generated/verification-runtime-trace.json"
     ("examples/runscope/run-001.trace.json")
     "Runtime traces are observed before interpretation.")
    ("evidence-graph:evidence" "evidence" "docs/generated/verification-evidence.json"
     ("docs/generated/verification-evidence-ops.json")
     "Generated backend receipts are durable evidence.")
    ("evidence-graph:interpretation" "interpretation" "docs/generated/verification-root-cause.json"
     ("docs/generated/verification-evidence-separation.json")
     "Interpretation is separated from raw evidence.")
    ("evidence-graph:claim" "claim" "docs/generated/verification-conformance.json"
     ("docs/generated/verification-semantic-diff.json")
     "Conformance is a claim backed by semantic diff.")
    ("evidence-graph:authority" "authority" "docs/generated/verification-authority.json"
     ("docs/generated/verification-evidence-quality.json")
     "Authority consumes assessed evidence, not agent confidence.")
    ("evidence-graph:feedback" "feedback" "docs/generated/verification-learning-loop.json"
     ("docs/generated/verification-feedback.json")
     "Closed feedback updates executable knowledge.")))

(defparameter *verification-evidence-graph-edges*
  '(("evidence-edge:runtime-evidence" "evidence-graph:runtime"
     "evidence-graph:evidence" "observes"
     ("docs/generated/verification-runtime-trace.json") "linked"
     "Runtime observations become stored evidence.")
    ("evidence-edge:evidence-interpretation" "evidence-graph:evidence"
     "evidence-graph:interpretation" "interprets"
     ("docs/generated/verification-evidence-separation.json") "linked"
     "Evidence cannot jump directly into claims.")
    ("evidence-edge:interpretation-claim" "evidence-graph:interpretation"
     "evidence-graph:claim" "supports"
     ("docs/generated/verification-root-cause.json") "linked"
     "Claims cite interpreted evidence.")
    ("evidence-edge:claim-authority" "evidence-graph:claim"
     "evidence-graph:authority" "gates"
     ("docs/generated/verification-authority.json") "linked"
     "Authority gates consume conformance claims.")
    ("evidence-edge:authority-feedback" "evidence-graph:authority"
     "evidence-graph:feedback" "updates"
     ("docs/generated/verification-learning-loop.json") "linked"
     "Approved authority produces learning feedback.")))

(defparameter *verification-evidence-graph-rules*
  '(("evidence-graph:no-orphans" "Every edge endpoint must be a graph node.")
    ("evidence-graph:evidence-exists" "Every cited evidence artifact must exist.")
    ("evidence-graph:feedback-closes" "Authority must route back to feedback.")))

(defun emit-verification-evidence-graph-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-evidence-graph out))))
    (if stream (write-string json stream) json)))

(defun write-verification-evidence-graph (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationevidencegraphgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_evidence_graph.lisp\",~%")
  (format out "  \"evidence_graph_profile\": \"linked-evidence-to-authority\",~%")
  (write-evidence-graph-nodes out) (format out ",~%")
  (write-evidence-graph-edges out) (format out ",~%")
  (write-evidence-graph-rules out) (format out "~%}~%"))

(defun write-evidence-graph-nodes (out)
  (format out "  \"nodes\": [~%")
  (loop for row in *verification-evidence-graph-nodes* for first = t then nil
        do (unless first (format out ",~%")) (write-evidence-graph-node out row))
  (format out "~%  ]"))
