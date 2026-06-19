(in-package #:dslraid.agent)

(defparameter *verification-adversarial-probes*
  '(("adversarial:ontology-mismatch" "release" "reviewer:red-team" "ontology-mismatch" "D2"
     ("missing-context" "wrong-version")
     ("docs/generated/verification-ontology.json" "docs/generated/verification-context-map.json")
     "review-required" "owner:ontology"
     "Find concepts mapped to the wrong ontology or context.")
    ("adversarial:evidence-gap" "conformance" "reviewer:red-team" "evidence-gap" "D2"
     ("unlinked-evidence" "claim-without-source")
     ("docs/generated/verification-evidence.json" "docs/generated/verification-evidence-quality.json")
     "review-required" "owner:evidence"
     "Find claims whose evidence chain is incomplete.")
    ("adversarial:authority-overreach" "authority" "reviewer:security" "authority-overreach" "D3"
     ("self-approval" "agent-authority")
     ("docs/generated/verification-authority.json" "docs/generated/verification-reasoning-access.json")
     "human-review" "owner:governance"
     "Find agents trying to turn reasoning into approval.")
    ("adversarial:rollback-gap" "release" "reviewer:release" "rollback-gap" "D3"
     ("missing-rollback" "stale-artifact")
     ("docs/generated/verification-release-provenance.json" "docs/generated/verification-security-audit.json")
     "human-review" "owner:release"
     "Find promotion paths that cannot be safely rolled back.")
    ("adversarial:hidden-assumption" "experiment" "reviewer:red-team" "hidden-assumption" "D1"
     ("unstated-assumption" "unreviewed-experiment")
     ("docs/generated/verification-pdca.json" "docs/generated/verification-evidence-ops.json")
     "review-required" "owner:pdca"
     "Find assumptions that experiments have not made executable.")))

(defparameter *verification-adversarial-rules*
  '(("adversarial:independent" "Adversarial reviewer is not the producer.")
    ("adversarial:failure-mode-named" "Every probe names what can fail.")
    ("adversarial:evidence-required" "Every probe cites generated evidence.")
    ("adversarial:severe-needs-human" "D3 and D4 probes require human review.")))

(defun emit-verification-adversarial-review-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-adversarial-review out))))
    (if stream (write-string json stream) json)))

(defun write-verification-adversarial-review (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationadversarialgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_adversarial_review.lisp\",~%")
  (format out "  \"adversarial_profile\": \"failure-mode-review\",~%")
  (write-adversarial-probes out)
  (format out ",~%")
  (write-adversarial-rules out)
  (format out "~%}~%"))

(defun write-adversarial-probes (out)
  (format out "  \"probes\": [~%")
  (loop for row in *verification-adversarial-probes* for first = t then nil
        do (unless first (format out ",~%")) (write-adversarial-probe out row))
  (format out "~%  ]"))

(defun write-adversarial-probe (out row)
  (destructuring-bind (id scope reviewer probe severity detects evidence effect owner meaning) row
    (format out "    {\"id\": \"~A\", \"scope\": \"~A\", " id scope)
    (format out "\"reviewer\": \"~A\", \"probe\": \"~A\", " reviewer probe)
    (format out "\"severity\": \"~A\", " severity)
    (write-authority-list out "detects" detects) (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"authority_effect\": \"~A\", \"owner\": \"~A\", " effect owner)
    (format out "\"meaning\": \"~A\"}" meaning)))

(defun write-adversarial-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-adversarial-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationadversarialgen.sh check\"}")))
  (format out "~%  ]"))
