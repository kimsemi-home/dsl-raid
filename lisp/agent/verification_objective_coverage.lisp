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
    ("objective:precommit" "local-precommit" "pre-commit enforces local quality gates"
     "gate:precommit" ("docs/generated/verification-precommit-closure.json")
     "Local commits are blocked by lint, tests, build, quality, and diff checks.")
    ("objective:generated-workflows" "backend-projection" "Verification Graph generates CI backends"
     "gate:backend-parity" ("docs/generated/verification-backend-parity.json" ".github/workflows/verification.yml" ".gitlab-ci.yml" "Makefile" "BUILD.bazel")
     "GitHub Actions, GitLab CI, Makefile, and Bazel project the same graph.")
    ("objective:codegen-chain" "codegen-chain" "Ontology to executable SSOT to generated artifacts"
     "gate:codegen" ("docs/generated/verification-ontology.json" "docs/generated/verification-codegen.json")
     "Code, docs, schemas, tests, conformance, actions, and releases are mapped.")
    ("objective:pdca" "pdca-learning" "collect evidence, experiment, review, and improve"
     "gate:pdca" ("docs/generated/verification-pdca.json" "docs/generated/verification-experiment-loop.json")
     "PDCA and experiments are explicit evidence before policy.")
    ("objective:learning-loop" "learning-loop" "mistakes become evidence-backed knowledge updates"
     "gate:learning-loop" ("docs/generated/verification-learning-loop.json")
     "Agent Cluster learning is tracked from reality to revalidation.")
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
