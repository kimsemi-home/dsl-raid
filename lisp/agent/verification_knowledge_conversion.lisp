(in-package #:dslraid.agent)

(defparameter *verification-knowledge-conversion-core*
  '((1 "knowledge-conversion:incompleteness" "incompleteness-visible"
     "lisp/agent/verification_incompleteness_ledger.lisp"
     "docs/generated/verification-incompleteness-ledger.json"
     "bash scripts/verificationincompletegen.sh check"
     "stdout:verification incompleteness ledger check ok"
     "gate:incompleteness-ledger"
     ("docs/generated/verification-incompleteness-ledger.json")
     "Incomplete knowledge is visible and owned.")
    (2 "knowledge-conversion:failure" "failure-signaled"
     "lisp/agent/verification_failure_conditions.lisp"
     "docs/generated/verification-failure-conditions.json"
     "bash scripts/verificationfailuregen.sh check"
     "stdout:verification failure conditions check ok"
     "gate:failure-conditions"
     ("docs/generated/verification-failure-conditions.json")
     "Failure signals block authority and name a response.")
    (3 "knowledge-conversion:evidence" "evidence-captured"
     "lisp/agent/verification_evidence.lisp"
     "docs/generated/verification-evidence.json"
     "bash scripts/verificationevidencegen.sh check"
     "stdout:verification evidence generated output ok"
     "gate:evidence-graph"
     ("docs/generated/verification-evidence.json")
     "The error becomes durable evidence.")
    (4 "knowledge-conversion:root-cause" "root-cause-mapped"
     "lisp/agent/verification_root_cause.lisp"
     "docs/generated/verification-root-cause.json"
     "bash scripts/verificationrootcausegen.sh check"
     "stdout:verification root cause check ok"
     "gate:root-cause"
     ("docs/generated/verification-root-cause.json")
     "Evidence narrows root cause candidates.")
    (5 "knowledge-conversion:debt" "debt-recorded"
     "lisp/agent/verification_debt.lisp" "docs/generated/verification-debt.json"
     "bash scripts/verificationdebtgen.sh check" "stdout:verification debt check ok"
     "gate:debt-register" ("docs/generated/verification-debt.json")
     "Unclosed learning gaps become explicit debt.")))

(defparameter *verification-knowledge-conversion-rules*
  '(("knowledge-conversion:visible" "Errors and unknowns must become visible evidence.")
    ("knowledge-conversion:owned" "Every unresolved learning gap has an owner.")
    ("knowledge-conversion:closed" "Conversion closes through versioned knowledge and revalidation.")))
