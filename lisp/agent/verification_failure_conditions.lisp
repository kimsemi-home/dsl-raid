(in-package #:dslraid.agent)

(defparameter *verification-failure-conditions*
  '(("failure:ontology-context" "ontology" "contextless-concept" "error" ("approve-change") ("docs/generated/verification-ontology.json" "docs/generated/verification-context-map.json") "response:add-context-version" "owner:ontology" "Concepts without context or version break ontology authority.")
    ("failure:confidence-self" "confidence" "self-confidence-final" "error" ("increase-confidence") ("docs/generated/verification-confidence.json" "docs/generated/verification-evidence-quality.json") "response:external-confidence" "owner:quality" "Agent confidence is not final confidence.")
    ("failure:reviewer-isolation" "reviewer" "producer-selected-reviewer" "warning" ("approve-review") ("docs/generated/verification-sidecar.json" "docs/generated/verification-review-capacity.json") "response:independent-review" "owner:review" "Reviewer agreement needs isolation and canonical artifacts.")
    ("failure:control-plane-manifest" "control-plane" "missing-orchestration-manifest" "error" ("route-work") ("docs/generated/verification-orchestration.json" "docs/generated/verification-authority.json") "response:create-manifest" "owner:control-plane" "Control plane decisions require manifest evidence.")
    ("failure:lease-missing" "lease" "no-active-lease" "error" ("commit-artifact") ("docs/generated/verification-lease.json" "docs/generated/verification-quarantine.json") "response:abort-or-rebase" "owner:lease" "Lease failures block artifact authority.")
    ("failure:translation-loss" "translation" "lossy-without-ledger" "error" ("promote-adapter") ("docs/generated/verification-loss-ledger.json" "docs/generated/verification-context-map.json") "response:record-loss" "owner:translation" "Lossy translation needs a loss ledger.")
    ("failure:evidence-quality-stale" "evidence-quality" "quality-snapshot-stale" "warning" ("raise-confidence") ("docs/generated/verification-evidence-quality.json" "docs/generated/verification-pruning.json") "response:reassess-quality" "owner:evidence" "Evidence quality snapshots must be refreshed after meaning changes.")
    ("failure:feedback-open" "feedback" "loop-gap-without-owner" "warning" ("close-incident") ("docs/generated/verification-feedback.json" "docs/generated/verification-codegen.json") "response:assign-owner" "owner:feedback" "Open feedback loops stay evidence until knowledge updates.")))

(defparameter *verification-failure-rules*
  '(("failure:blocks-explicit" "Every failure condition declares blocked authority.")
    ("failure:evidence-linked" "Failure conditions link generated evidence.")
    ("failure:owner-required" "Each failure condition has a non-agent owner.")))

(defun emit-verification-failure-json (&optional stream)
  "Emit failure condition predicates for the verification graph."
  (let ((json (with-output-to-string (out)
                (write-verification-failure out))))
    (if stream (write-string json stream) json)))

(defun write-verification-failure (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationfailuregen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification.lisp\",~%")
  (format out "  \"failure_profile\": \"operational-breakage\",~%")
  (write-failure-conditions out)
  (format out ",~%")
  (write-failure-rules out)
  (format out "~%}~%"))

(defun write-failure-conditions (out)
  (format out "  \"conditions\": [~%")
  (loop for row in *verification-failure-conditions*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-failure-condition out row))
  (format out "~%  ]"))

(defun write-failure-condition (out row)
  (destructuring-bind (id domain signal severity blocks evidence response owner meaning) row
    (format out "    {\"id\": \"~A\", \"domain\": \"~A\", " id domain)
    (format out "\"signal\": \"~A\", \"severity\": \"~A\", " signal severity)
    (write-authority-list out "blocks" blocks) (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"response\": \"~A\", \"owner\": \"~A\", " response owner)
    (format out "\"meaning\": \"~A\"}" meaning)))

(defun write-failure-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-failure-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationfailuregen.sh check\"}")))
  (format out "~%  ]"))
