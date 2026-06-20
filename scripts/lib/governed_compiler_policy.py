generated_by = "scripts/verificationcompilergen.sh"
profile = "spec-candidate-evidence-authority"
source = "lisp/agent/verification_governed_compiler.lisp"
subject = "verify:daemon"

required_stages = [
    "spec",
    "candidate",
    "validation",
    "evidence",
    "external-confidence",
    "authority",
]

required_rules = {
    "governed-compiler:candidate-not-result",
    "governed-compiler:deterministic-checks",
    "governed-compiler:authority-last",
}
