(in-package #:dslraid.agent)

(defparameter *verification-learning-stages*
  '(("stage:reality" 1 "runtime-event" "observation"
     ("docs/generated/verification-runtime-trace.json")
     "Reality is captured before interpretation.")
    ("stage:observation" 2 "observation" "evidence"
     ("docs/generated/verification-evidence.json")
     "Observed facts become durable evidence.")
    ("stage:evidence" 3 "evidence" "interpretation"
     ("docs/generated/verification-evidence-separation.json")
     "Evidence is separated from claims and interpretations.")
    ("stage:interpretation" 4 "interpretation" "rulebook"
     ("docs/generated/verification-root-cause.json")
     "Interpretation stays bounded until validated.")
    ("stage:rulebook" 5 "rulebook" "design"
     ("docs/generated/verification-context-map.json")
     "Rules and contexts update before implementation.")
    ("stage:design" 6 "design" "codegen"
     ("docs/generated/verification-ssot-defect.json")
     "Design updates name affected SSOT surfaces.")
    ("stage:codegen" 7 "codegen" "revalidation"
     ("docs/generated/verification-codegen.json"
      "docs/generated/verification-conformance.json")
     "Generated artifacts regain authority only after checks.")))

(defparameter *verification-learning-cycles*
  '(("learning:generated-drift" "trigger:generated-drift"
     "incident:generated-drift-drill"
     ("stage:reality" "stage:observation" "stage:evidence"
      "stage:interpretation" "stage:rulebook" "stage:design" "stage:codegen")
     ("docs/generated/verification-incident-learning.json"
      "docs/generated/verification-semantic-diff.json")
     "owner:verification" "update:generated-freshness-rule"
     "revalidate:quality-gate" "closed"
     "Generated drift becomes a learning cycle, not an untracked fix.")
    ("learning:agent-claim" "trigger:unchecked-agent-claim"
     "incident:claim-confidence-gap"
     ("stage:observation" "stage:evidence" "stage:interpretation"
      "stage:rulebook" "stage:codegen")
     ("docs/generated/verification-confidence.json"
      "docs/generated/verification-actions-receipt.json")
     "owner:quality" "update:confidence-ceiling-rule"
     "revalidate:actions-receipt" "closed"
     "Agent claims need evidence and external confidence before authority.")))

(defparameter *verification-learning-rules*
  '(("learning-loop:ordered" "Learning stages must remain ordered.")
    ("learning-loop:evidence-linked" "Stages and cycles cite generated evidence.")
    ("learning-loop:update-required" "Closed cycles name knowledge updates.")
    ("learning-loop:owner-required" "Cycle ownership is non-agent.")))
