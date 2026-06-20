(in-package #:dslraid.agent)

(defparameter *verification-semantic-ssot-diffs*
  '(("semantic-diff:evidence-before-change" "semantic:evidence-before-change"
     "Evidence-before-change semantic receipt.")
    ("semantic-diff:versioned-ssot" "semantic:versioned-ssot"
     "Versioned SSOT semantic receipt.")
    ("semantic-diff:bounded-context" "semantic:bounded-context"
     "Bounded context semantic receipt.")
    ("semantic-diff:migration-surface" "semantic:migration-surface"
     "Migration and compatibility command surface semantic receipt.")
    ("semantic-diff:language-expansion" "semantic:language-expansion"
     "Language expansion semantic receipt.")
    ("semantic-diff:runtime-contract" "semantic:runtime-contract"
     "Runtime contract semantic receipt.")
    ("semantic-diff:context-map" "semantic:context-map"
     "Context map semantic receipt.")
    ("semantic-diff:translation-verifier" "semantic:translation-verifier"
     "Translation verifier semantic receipt.")
    ("semantic-diff:historical-interpreter" "semantic:historical-interpreter"
     "Historical interpreter semantic receipt.")
    ("semantic-diff:ontology-transition" "semantic:ontology-transition"
     "Ontology transition semantic receipt.")
    ("semantic-diff:ssot-defect" "semantic:ssot-defect"
     "SSOT defect semantic receipt.")
    ("semantic-diff:root-cause" "semantic:root-cause"
     "Root cause semantic receipt.")
    ("semantic-diff:semantic-debugger" "semantic:semantic-debugger"
     "Semantic debugger semantic receipt.")
    ("semantic-diff:evidence-pruning" "semantic:evidence-pruning"
     "Evidence pruning semantic receipt.")
    ("semantic-diff:security-audit" "semantic:security-audit"
     "Security audit semantic receipt.")
    ("semantic-diff:failure-conditions" "semantic:failure-conditions"
     "Failure conditions semantic receipt.")
    ("semantic-diff:debt-register" "semantic:debt-register"
     "Debt register semantic receipt.")))
