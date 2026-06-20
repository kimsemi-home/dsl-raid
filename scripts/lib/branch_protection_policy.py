generated_by = "scripts/verificationbranchgen.sh"
profile = "main-required-checks"
source = "lisp/agent/verification_branch_protection.lisp"
subject = "verify:daemon"

required_checks = {
    "CI",
    "Security",
    "Golden",
    "Verification Graph",
}

required_rules = {
    "branch-protect:main-only",
    "branch-protect:required-checks",
    "branch-protect:no-target-event",
    "branch-protect:evidence-linked",
}
