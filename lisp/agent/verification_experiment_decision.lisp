(in-package #:dslraid.agent)

(defparameter *verification-experiment-decisions*
  '(("experiment-decision:bootstrap-sequence" "experiment:bootstrap-sequence"
     "promote" "gate:release-check" "docs/generated/verification-conformance.json"
     ("docs/generated/verification-experiment-loop.json"
      "docs/generated/verification-bootstrap-sequence.json"
      "docs/generated/verification-semantic-diff.json")
     "closed" "Bootstrap sequence moved from experiment to release-gated evidence.")
    ("experiment-decision:run-manifest" "experiment:run-manifest"
     "promote" "gate:evidence-quality" "docs/generated/verification-evidence-quality.json"
     ("docs/generated/verification-experiment-loop.json"
      "docs/generated/verification-run-manifest.json"
      "docs/generated/verification-evidence-quality.json")
     "closed" "Run manifests moved from experiment to quality-gated evidence.")))

(defparameter *verification-experiment-decision-rules*
  '(("experiment-decision:known-experiment"
     "Every decision points back to an experiment-loop record.")
    ("experiment-decision:act-matches"
     "The decision act artifact must match the experiment act target.")
    ("experiment-decision:promote-after-check"
     "Promotion is allowed only after checked experiment evidence.")))

(defun emit-verification-experiment-decision-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-experiment-decision out))))
    (if stream (write-string json stream) json)))

(defun write-verification-experiment-decision (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationexperimentdecisiongen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_experiment_decision.lisp\",~%")
  (format out "  \"experiment_decision_profile\": \"hypothesis-to-act-closure\",~%")
  (write-experiment-decisions out)
  (format out ",~%")
  (write-experiment-decision-rules out)
  (format out "~%}~%"))

(defun write-experiment-decisions (out)
  (format out "  \"decisions\": [~%")
  (loop for row in *verification-experiment-decisions* for first = t then nil
        do (unless first (format out ",~%"))
           (write-experiment-decision out row))
  (format out "~%  ]"))

(defun write-experiment-decision (out row)
  (destructuring-bind (id experiment decision gate act evidence status meaning) row
    (format out "    {\"id\": \"~A\", \"experiment\": \"~A\", " id experiment)
    (format out "\"decision\": \"~A\", \"gate\": \"~A\", " decision gate)
    (format out "\"act\": \"~A\", " act)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"~A\", \"meaning\": \"~A\"}" status meaning)))

(defun write-experiment-decision-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-experiment-decision-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationexperimentdecisiongen.sh check\"}")))
  (format out "~%  ]"))
