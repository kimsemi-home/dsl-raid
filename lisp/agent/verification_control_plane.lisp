(in-package #:dslraid.agent)

(defparameter *verification-control-plane-routes*
  '(("control-plane:release-route" "route:release-check" "control-plane:primary"
     "sidecar:control-plane-verifier" "shadow:release-orchestrator" "D2" "review"
     "authority:governance" ("docs/generated/verification-orchestration.json"
                             "docs/generated/verification-sidecar.json")
     "Release routing is checked by an independent verifier and shadow route.")
    ("control-plane:authority-route" "route:authority-gate" "control-plane:primary"
     "sidecar:control-plane-verifier" "shadow:authority-orchestrator" "D3"
     "human-review" "authority:governance"
     ("docs/generated/verification-authority.json"
      "docs/generated/verification-review-capacity.json")
     "Authority routing divergence requires human governance review.")
    ("control-plane:policy-route" "route:policy-boundary" "control-plane:primary"
     "sidecar:control-plane-verifier" "shadow:policy-orchestrator" "D4"
     "authority-blocked" "authority:security"
     ("docs/generated/verification-access-policy.json"
      "docs/generated/verification-security-audit.json")
     "Policy hash or trust-boundary divergence blocks control-plane authority.")))

(defparameter *verification-control-plane-rules*
  '(("control-plane:verifier-separated" "Control plane cannot verify itself.")
    ("control-plane:shadow-required" "High-risk routes require a shadow route.")
    ("control-plane:divergence-gated" "D3 and D4 divergence require authority gates.")))

(defun emit-verification-control-plane-json (&optional stream)
  "Emit control-plane verifier receipts."
  (let ((json (with-output-to-string (out)
                (write-verification-control-plane out))))
    (if stream (write-string json stream) json)))

(defun write-verification-control-plane (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationcontrolgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_control_plane.lisp\",~%")
  (format out "  \"control_plane_profile\": \"independent-shadow-verifier\",~%")
  (write-control-plane-routes out)
  (format out ",~%")
  (write-control-plane-rules out)
  (format out "~%}~%"))

(defun write-control-plane-routes (out)
  (format out "  \"routes\": [~%")
  (loop for row in *verification-control-plane-routes*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-control-plane-route out row))
  (format out "~%  ]"))

(defun write-control-plane-route (out row)
  (destructuring-bind (id route control verifier shadow divergence severity authority evidence meaning) row
    (format out "    {\"id\": \"~A\", \"route\": \"~A\", " id route)
    (format out "\"control_plane\": \"~A\", \"verifier\": \"~A\", " control verifier)
    (format out "\"shadow\": \"~A\", \"divergence\": \"~A\", " shadow divergence)
    (format out "\"severity\": \"~A\", \"authority\": \"~A\", " severity authority)
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-control-plane-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-control-plane-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationcontrolgen.sh check\"}")))
  (format out "~%  ]"))
