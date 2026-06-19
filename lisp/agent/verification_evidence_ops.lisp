(in-package #:dslraid.agent)

(defparameter *verification-evidence-ops-records*
  '(("evidence-ops:ci" "ci" "evidence-generator" "verify:daemon"
     ("docs/generated/verification-evidence.json" "docs/generated/verification-github-actions.json")
     ("docs/generated/verification-evidence.json") "evidence-only"
     "owner:verification"
     "CI emits evidence and never substitutes for governance authority.")
    ("evidence-ops:quality" "quality-gate" "conformance-claim" "verify:daemon"
     ("docs/generated/verification-conformance.json" "docs/generated/verification-semantic-diff.json")
     ("docs/generated/verification-conformance.json") "review-required"
     "owner:quality"
     "Quality gates turn checks into reviewable conformance evidence.")
    ("evidence-ops:experiment" "experiment" "pdca-experiment" "verify:daemon"
     ("docs/generated/verification-pdca.json" "docs/generated/verification-reasoning-access.json")
     ("docs/generated/verification-pdca.json") "review-required"
     "owner:pdca"
     "Experiments must create PDCA evidence before becoming policy.")
    ("evidence-ops:release" "release" "conformance-claim" "verify:daemon"
     ("docs/generated/verification-release-provenance.json" "docs/generated/verification-evidence-quality.json")
     ("docs/generated/verification-release-provenance.json") "release-gated"
     "owner:release"
     "Release deploys are conformance claims with provenance evidence.")
    ("evidence-ops:incident" "incident" "knowledge-update" "verify:daemon"
     ("docs/generated/verification-incident-learning.json" "docs/generated/verification-feedback.json")
     ("docs/generated/verification-feedback.json") "learning-loop"
     "owner:learning"
     "Operations close only when evidence updates executable knowledge.")))

(defparameter *verification-evidence-ops-rules*
  '(("evidence-ops:ci-not-authority" "CI emits evidence; governance owns authority.")
    ("evidence-ops:deploy-is-claim" "Deploy surfaces are conformance claims.")
    ("evidence-ops:update-graph" "Operations update the evidence graph.")
    ("evidence-ops:experiment-reviewed" "Experiments require review before promotion.")))

(defun emit-verification-evidence-ops-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-evidence-ops out))))
    (if stream (write-string json stream) json)))

(defun write-verification-evidence-ops (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationevidenceopsgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_evidence_ops.lisp\",~%")
  (format out "  \"evidence_ops_profile\": \"ci-deploy-evidence-loop\",~%")
  (write-evidence-ops-records out)
  (format out ",~%")
  (write-evidence-ops-rules out)
  (format out "~%}~%"))

(defun write-evidence-ops-records (out)
  (format out "  \"records\": [~%")
  (loop for row in *verification-evidence-ops-records* for first = t then nil
        do (unless first (format out ",~%")) (write-evidence-ops-record out row))
  (format out "~%  ]"))

(defun write-evidence-ops-record (out row)
  (destructuring-bind (id operation claim subject evidence updates effect owner meaning) row
    (format out "    {\"id\": \"~A\", \"operation\": \"~A\", " id operation)
    (format out "\"claim\": \"~A\", \"subject\": \"~A\", " claim subject)
    (write-authority-list out "evidence" evidence) (format out ", ")
    (write-authority-list out "updates" updates)
    (format out ", \"authority_effect\": \"~A\", " effect)
    (format out "\"owner\": \"~A\", \"meaning\": \"~A\"}" owner meaning)))

(defun write-evidence-ops-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-evidence-ops-rules* for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationevidenceopsgen.sh check\"}")))
  (format out "~%  ]"))
