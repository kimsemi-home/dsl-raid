(in-package #:dslraid.agent)

(defparameter *verification-semantic-debug-sessions*
  '(("semantic-debug:ssot-defect-drill" "observation:ssot-defect-drill"
     ("docs/generated/verification-evidence.json"
      "docs/generated/verification-root-cause.json"
      "docs/generated/verification-semantic-hash.json"
      "docs/generated/verification-orchestration.json"
      "docs/generated/verification-evidence-quality.json")
     ("ontology:verification-graph")
     ("cause:contract-drift" "cause:generation-drift")
     ("evidence:runtime-reproduction" "evidence:contract-probe")
     ("version-conflict:none-observed")
     ("loss:manifest-schema")
     "medium"
     ("docs/generated/verification-conformance.json"
      "docs/generated/verification-semantic-diff.json")
     "medium" "authority-gate"
     ("docs/generated/verification-root-cause.json"
      "docs/generated/verification-loss-ledger.json")
     "Semantic debugger narrows questions before any patch authority.")))
(defparameter *verification-semantic-debug-rules*
  '(("semantic-debugger:inputs-linked" "Debugger inputs must be generated evidence.")
    ("semantic-debugger:missing-evidence-declared" "Unanswered questions stay explicit.")
    ("semantic-debugger:no-final-cause" "Debugger sessions do not confirm root cause.")))
(defun emit-verification-semantic-debugger-json (&optional stream)
  "Emit semantic debugger question set for verification evidence."
  (let ((json (with-output-to-string (out)
                (write-verification-semantic-debugger out))))
    (if stream (write-string json stream) json)))
(defun write-verification-semantic-debugger (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationdebuggergen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"semantic_debugger_profile\": \"question-oriented\",~%")
  (write-semantic-debug-sessions out)
  (format out ",~%")
  (write-semantic-debug-rules out)
  (format out "~%}~%"))
(defun write-semantic-debug-sessions (out)
  (format out "  \"sessions\": [~%")
  (loop for row in *verification-semantic-debug-sessions*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-semantic-debug-session out row))
  (format out "~%  ]"))
(defun write-semantic-debug-session (out row)
  (destructuring-bind (id obs inputs affected causes missing conflicts loss risk required confidence escalation evidence meaning) row
    (format out "    {\"id\": \"~A\", \"observation\": \"~A\", " id obs)
    (write-authority-list out "inputs" inputs) (format out ", ")
    (write-authority-list out "affected_ontology" affected) (format out ", ")
    (write-authority-list out "possible_causes" causes) (format out ", ")
    (write-authority-list out "missing_evidence" missing) (format out ", ")
    (write-authority-list out "version_conflicts" conflicts) (format out ", ")
    (write-authority-list out "translation_loss" loss) (format out ", ")
    (format out "\"evidence_quality_risk\": \"~A\", " risk)
    (write-authority-list out "required_verification" required)
    (format out ", \"confidence_ceiling\": \"~A\", " confidence)
    (format out "\"escalation\": \"~A\", " escalation)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))
(defun write-semantic-debug-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-semantic-debug-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationdebuggergen.sh check\"}")))
  (format out "~%  ]"))
