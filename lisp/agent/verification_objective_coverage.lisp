(in-package #:dslraid.agent)

(defparameter *verification-objective-coverage*
  '(("objective:privacy" "privacy-exclusion" "public artifacts exclude personal data"
     "gate:privacy" ("docs/generated/verification-privacy.json" "scripts/privacycheck.sh")
     "Privacy exclusion is tracked by generated privacy evidence.")
    ("objective:auto-merge" "autonomous-merge" "merge and push through guarded automation"
     "gate:merge-readiness" ("docs/generated/verification-merge-readiness.json" "docs/generated/verification-merge-automation.json")
     "Autonomous merge work is gated by readiness and automation policy.")
    ("objective:line-budget" "source-shape" "source files stay within 75 lines"
     "gate:source-shape" ("docs/generated/verification-source-shape.json")
     "Line budget and public surface are tracked as source shape evidence.")
    ("objective:generated-workflows" "backend-projection" "Verification Graph generates CI backends"
     "gate:backend-parity" ("docs/generated/verification-backend-parity.json" ".github/workflows/verification.yml" ".gitlab-ci.yml" "Makefile" "BUILD.bazel")
     "GitHub Actions, GitLab CI, Makefile, and Bazel project the same graph.")
    ("objective:codegen-chain" "codegen-chain" "Ontology to executable SSOT to generated artifacts"
     "gate:codegen" ("docs/generated/verification-ontology.json" "docs/generated/verification-codegen.json")
     "Code, docs, schemas, tests, conformance, actions, and releases are mapped.")
    ("objective:pdca" "pdca-learning" "collect evidence, experiment, review, and improve"
     "gate:pdca" ("docs/generated/verification-pdca.json" "docs/generated/verification-experiment-loop.json")
     "PDCA and experiments are explicit evidence before policy.")
    ("objective:remote-actions" "actions-receipt" "remote GitHub Actions receipt"
     "gate:actions-receipt" ("docs/generated/verification-actions-receipt.json")
     "Remote run result is tracked by head SHA, conclusion, and URL.")
    ("objective:query-lazy" "query-lazy-surface" "query and lazy composition are executable surfaces"
     "gate:query-surface" ("docs/generated/verification-query-surface.json")
     "Query and lazy composition are command-backed, not hidden implementation details.")))

(defparameter *verification-objective-coverage-rules*
  '(("objective-coverage:evidence-linked" "Every objective row cites evidence.")
    ("objective-coverage:no-completion-claim" "Coverage tracks evidence without declaring final completion.")
    ("objective-coverage:gates-present" "Named gates must map to generated manifests.")))

(defun emit-verification-objective-coverage-json (&optional stream)
  (let ((json (with-output-to-string (out) (write-verification-objective-coverage out))))
    (if stream (write-string json stream) json)))

(defun write-verification-objective-coverage (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationobjectivegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_objective_coverage.lisp\",~%")
  (format out "  \"objective_coverage_profile\": \"active-goal-trace\",~%")
  (write-objective-coverage-items out)
  (format out ",~%")
  (write-objective-coverage-rules out)
  (format out "~%}~%"))

(defun write-objective-coverage-items (out)
  (format out "  \"requirements\": [~%")
  (loop for row in *verification-objective-coverage* for first = t then nil
        do (unless first (format out ",~%")) (write-objective-coverage-item out row))
  (format out "~%  ]"))

(defun write-objective-coverage-item (out row)
  (destructuring-bind (id kind requirement gate evidence meaning) row
    (format out "    {\"id\": \"~A\", \"kind\": \"~A\", " id kind)
    (format out "\"requirement\": \"~A\", \"gate\": \"~A\", " requirement gate)
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"tracked\", \"meaning\": \"~A\"}" meaning)))

(defun write-objective-coverage-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-objective-coverage-rules* for first = t then nil
        do (unless first (format out ",~%")) (write-objective-coverage-rule out row))
  (format out "~%  ]"))

(defun write-objective-coverage-rule (out row)
  (destructuring-bind (id meaning) row
    (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
    (format out "\"check\": \"scripts/verificationobjectivegen.sh check\"}")))
