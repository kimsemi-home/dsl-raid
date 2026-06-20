(in-package #:dslraid.agent)

(defparameter *verification-confidence-decisions*
  '(("confidence-decision:verification-output" "confidence:verification-output"
     "raise" "gate:evidence-quality"
     ("docs/generated/verification-evidence-quality.json"
      "docs/generated/verification-authority.json"
      "docs/generated/verification-confidence.json")
     ("evidence-quality" "authority-gate") "bounded-auto" "closed"
     "Output confidence may rise only after governed evidence and authority checks.")
    ("confidence-decision:release-check" "confidence:release-check"
     "raise" "gate:release"
     ("docs/generated/verification-semantic-diff.json"
      "docs/generated/verification-review-capacity.json"
      "docs/generated/verification-experiment-decision.json")
     ("semantic-diff" "review-capacity" "experiment-decision")
     "release-eligible" "closed"
     "Release confidence rises when semantic review and experiment decisions close.")))

(defparameter *verification-confidence-decision-rules*
  '(("confidence-decision:known-ceiling"
     "Each confidence decision references a confidence ceiling.")
    ("confidence-decision:external-gate"
     "Confidence decisions are made by gates, not by agent self-confidence.")
    ("confidence-decision:evidence-linked"
     "Each confidence decision cites generated evidence.")))

(defun emit-verification-confidence-decision-json (&optional stream)
  (let ((json (with-output-to-string (out)
                (write-verification-confidence-decision out))))
    (if stream (write-string json stream) json)))

(defun write-verification-confidence-decision (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationconfidencedecisiongen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_confidence_decision.lisp\",~%")
  (format out "  \"confidence_decision_profile\": \"evidence-to-confidence-change\",~%")
  (write-confidence-decisions out) (format out ",~%")
  (write-confidence-decision-rules out) (format out "~%}~%"))

(defun write-confidence-decisions (out)
  (format out "  \"decisions\": [~%")
  (loop for row in *verification-confidence-decisions* for first = t then nil
        do (unless first (format out ",~%")) (write-confidence-decision out row))
  (format out "~%  ]"))

(defun write-confidence-decision (out row)
  (destructuring-bind (id ceiling decision gate evidence requires effect status meaning) row
    (format out "    {\"id\": \"~A\", \"ceiling\": \"~A\", " id ceiling)
    (format out "\"decision\": \"~A\", \"gate\": \"~A\", " decision gate)
    (write-authority-list out "evidence" evidence) (format out ", ")
    (write-authority-list out "requires" requires)
    (format out ", \"authority_effect\": \"~A\", \"status\": \"~A\", " effect status)
    (format out "\"meaning\": \"~A\"}" meaning)))

(defun write-confidence-decision-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-confidence-decision-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationconfidencedecisiongen.sh check\"}")))
  (format out "~%  ]"))
