generated_by = "scripts/verificationevidencebeforechangegen.sh"
profile = "linked-evidence-or-debt"
source = "lisp/agent/verification.lisp"
subject = "verify:daemon"

change_kinds = {"routine", "emergency"}

required_changes = [
    ("change:generated-verification-graph", "routine"),
    ("change:release-routing", "routine"),
    ("change:emergency-patch", "emergency"),
]

required_rules = {
    "evidence-before-change:linked",
    "evidence-before-change:debt",
    "evidence-before-change:authority",
}
