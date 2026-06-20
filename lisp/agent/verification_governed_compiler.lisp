(in-package #:dslraid.agent)

(defparameter *verification-governed-compiler-stages*
  '((1 "governed-compiler:spec" "spec" "examples/runscope/runscope.assertions.json"
     "docs/generated/assertion-catalog.md"
     "bash scripts/assertiongen.sh check"
     "stdout:assertion generated doc ok" "source"
     ("examples/runscope/runscope.assertions.json")
     "Spec is the input to interpretation, not an agent opinion.")
    (2 "governed-compiler:candidate" "candidate"
     "lisp/agent/verification_executable_knowledge.lisp"
     "docs/generated/verification-executable-knowledge.json"
     "bash scripts/verificationexecutablegen.sh check"
     "stdout:verification executable knowledge check ok" "candidate"
     ("docs/generated/verification-executable-knowledge.json")
     "Agent-generated interpretation remains a candidate until checked.")
    (3 "governed-compiler:validation" "validation"
     "docs/generated/verification-executable-knowledge.json"
     "docs/generated/verification-conformance.json"
     "bash scripts/verificationconformancegen.sh check"
     "stdout:verification conformance generated output ok" "checked"
     ("schemas/dslraid-verification-manifest.schema.json")
     "Validation turns candidate interpretation into checked evidence.")
    (4 "governed-compiler:evidence" "evidence"
     "docs/generated/verification-conformance.json"
     "docs/generated/verification-evidence.json"
     "bash scripts/verificationevidencegen.sh check"
     "stdout:verification evidence generated output ok" "evidence"
     ("docs/generated/verification-semantic-hash.json")
     "Evidence is generated separately from the candidate output.")
    (5 "governed-compiler:external-confidence" "external-confidence"
     "docs/generated/verification-evidence.json"
     "docs/generated/verification-confidence.json"
     "bash scripts/verificationconfidencegen.sh check"
     "stdout:verification confidence check ok" "assessed"
     ("docs/generated/verification-evidence-quality.json")
     "External confidence is assessed from evidence, not self-reported.")
    (6 "governed-compiler:authority" "authority"
     "docs/generated/verification-confidence.json"
     "docs/generated/verification-authority.json"
     "bash scripts/verificationauthoritygen.sh check"
     "stdout:verification authority check ok" "gated"
     ("docs/generated/verification-access-policy.json")
     "Authority gate decides what may merge or release.")))

(defparameter *verification-governed-compiler-rules*
  '(("governed-compiler:candidate-not-result" "Agent output is a candidate.")
    ("governed-compiler:deterministic-checks" "Each stage has command evidence.")
    ("governed-compiler:authority-last" "Authority follows evidence and confidence.")))
