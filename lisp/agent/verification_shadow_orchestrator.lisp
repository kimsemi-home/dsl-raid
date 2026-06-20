(in-package #:dslraid.agent)

(defparameter *verification-shadow-routes*
  '(("shadow:release-check" "route:release-check" "control-plane:primary"
     "shadow:release-orchestrator" "D2" "observe"
     ("docs/generated/verification-control-plane.json"
      "docs/generated/verification-orchestration.json")
     "Release shadow routing observes divergence without changing authority.")
    ("shadow:authority-gate" "route:authority-gate" "control-plane:primary"
     "shadow:authority-orchestrator" "D3" "human-review"
     ("docs/generated/verification-control-plane.json"
      "docs/generated/verification-review-capacity.json")
     "Authority shadow divergence requires human review before promotion.")
    ("shadow:policy-boundary" "route:policy-boundary" "control-plane:primary"
     "shadow:policy-orchestrator" "D4" "authority-blocked"
     ("docs/generated/verification-control-plane.json"
      "docs/generated/verification-security-audit.json")
     "Policy shadow divergence blocks authority until evidence is reconciled.")))

(defparameter *verification-shadow-rules*
  '(("shadow-orchestrator:separated" "Shadow route differs from primary route.")
    ("shadow-orchestrator:divergence-gated" "D3 and D4 divergence gates authority.")
    ("shadow-orchestrator:evidence-linked" "Every shadow route cites generated evidence.")))

(defun emit-verification-shadow-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-shadow out))))
    (if stream (write-string json stream) json)))

(defun write-verification-shadow (out)
  (format out "{~%  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationshadowgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_shadow_orchestrator.lisp\",~%")
  (format out "  \"shadow_orchestrator_profile\": \"independent-observer\",~%")
  (write-shadow-routes out)
  (format out ",~%")
  (write-shadow-rules out)
  (format out "~%}~%"))

(defun write-shadow-routes (out)
  (format out "  \"routes\": [~%")
  (loop for row in *verification-shadow-routes* for first = t then nil
        do (unless first (format out ",~%")) (write-shadow-route out row))
  (format out "~%  ]"))

(defun write-shadow-route (out row)
  (destructuring-bind (id route primary shadow divergence action evidence meaning) row
    (format out "    {\"id\": \"~A\", \"route\": \"~A\", " id route)
    (format out "\"primary\": \"~A\", \"shadow\": \"~A\", " primary shadow)
    (format out "\"divergence\": \"~A\", \"action\": \"~A\", " divergence action)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-shadow-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-shadow-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationshadowgen.sh check\"}")))
  (format out "~%  ]"))
