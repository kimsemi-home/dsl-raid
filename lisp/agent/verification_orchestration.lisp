(in-package #:dslraid.agent)

(defparameter *verification-orchestration-routes*
  '(("orchestration:release-check" "decision:release-check-route"
     "control-plane:verification" "agent:verification-daemon"
     "policy:verification-graph" "gate:release"
     ("docs/generated/verification-evidence.json"
      "docs/generated/verification-sidecar.json")
     (".github/workflows/verification.yml" ".gitlab-ci.yml" "Makefile"
      "BUILD.bazel")
     "Release routing is authoritative only with policy, agent, evidence, and outputs.")
    ("orchestration:conformance" "decision:conformance-route"
     "control-plane:verification" "agent:quality-runner"
     "policy:quality-gate" "gate:quality"
     ("docs/generated/verification-conformance.json"
      "docs/generated/verification-evidence-quality.json")
     ("docs/generated/verification-conformance.json"
      "tests/golden/verification-graph.generated.json")
     "Conformance routing records the quality gate evidence and outputs.")))

(defparameter *verification-orchestration-rules*
  '(("orchestration:policy-present" "Every route names policy.")
    ("orchestration:agent-present" "Every route names agent.")
    ("orchestration:evidence-output-present" "Every route links evidence and outputs.")))

(defun emit-verification-orchestration-json (&optional stream)
  "Emit orchestration receipts for verification control-plane routes."
  (let ((json (with-output-to-string (out)
                (write-verification-orchestration out))))
    (if stream (write-string json stream) json)))

(defun write-verification-orchestration (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationorchestrationgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"orchestration_profile\": \"policy-bound-routing\",~%")
  (write-orchestration-routes out)
  (format out ",~%")
  (write-orchestration-rules out)
  (format out "~%}~%"))

(defun write-orchestration-routes (out)
  (format out "  \"routes\": [~%")
  (loop for row in *verification-orchestration-routes*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-orchestration-route out row))
  (format out "~%  ]"))

(defun write-orchestration-route (out row)
  (destructuring-bind (id decision plane agent policy authority evidence outputs meaning) row
    (format out "    {\"id\": \"~A\", \"decision\": \"~A\", " id decision)
    (format out "\"control_plane\": \"~A\", \"agent\": \"~A\", " plane agent)
    (format out "\"policy\": \"~A\", \"authority\": \"~A\", " policy authority)
    (write-authority-list out "evidence" evidence)
    (format out ", ")
    (write-authority-list out "outputs" outputs)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-orchestration-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-orchestration-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", " id)
             (format out "\"meaning\": \"~A\", " meaning)
             (format out "\"check\": \"scripts/verificationorchestrationgen.sh check\"}")))
  (format out "~%  ]"))
