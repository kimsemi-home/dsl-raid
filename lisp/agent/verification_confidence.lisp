(in-package #:dslraid.agent)

(defparameter *verification-confidence-ceilings*
  '(("confidence:verification-output" "agent:verification-daemon" "ignored"
     "high" "gate:evidence-quality"
     ("evidence-quality" "authority-gate" "quarantine")
     ("docs/generated/verification-evidence-quality.json"
      "docs/generated/verification-authority.json"
      "docs/generated/verification-quarantine.json")
     "Agent output confidence is capped by evidence and governance gates.")
    ("confidence:release-check" "control-plane:verification" "ignored"
     "high" "gate:release"
     ("semantic-diff" "review-capacity" "feedback-closure" "experiment-decision")
     ("docs/generated/verification-semantic-diff.json"
      "docs/generated/verification-review-capacity.json"
      "docs/generated/verification-feedback.json"
      "docs/generated/verification-experiment-decision.json")
     "Release confidence comes from review and semantic evidence.")))

(defparameter *verification-confidence-rules*
  '(("confidence:self-ignored" "Agent self confidence is never authoritative.")
    ("confidence:decider-external" "A non-agent gate decides confidence ceiling.")
    ("confidence:evidence-linked" "Every ceiling links generated evidence.")))

(defun emit-verification-confidence-json (&optional stream)
  "Emit confidence sidecar for externally bounded verification confidence."
  (let ((json (with-output-to-string (out)
                (write-verification-confidence out))))
    (if stream (write-string json stream) json)))

(defun write-verification-confidence (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationconfidencegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"confidence_profile\": \"external-evidence-ceiling\",~%")
  (write-confidence-ceilings out)
  (format out ",~%")
  (write-confidence-rules out)
  (format out "~%}~%"))

(defun write-confidence-ceilings (out)
  (format out "  \"ceilings\": [~%")
  (loop for row in *verification-confidence-ceilings*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-confidence-ceiling out row))
  (format out "~%  ]"))

(defun write-confidence-ceiling (out row)
  (destructuring-bind (id producer self ceiling decider requires evidence meaning) row
    (format out "    {\"id\": \"~A\", \"producer\": \"~A\", " id producer)
    (format out "\"self_confidence\": \"~A\", \"ceiling\": \"~A\", " self ceiling)
    (format out "\"decided_by\": \"~A\", " decider)
    (write-authority-list out "requires" requires)
    (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-confidence-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-confidence-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationconfidencegen.sh check\"}")))
  (format out "~%  ]"))
