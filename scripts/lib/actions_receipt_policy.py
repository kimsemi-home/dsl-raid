generated_by = "scripts/verificationreceiptgen.sh"
profile = "remote-run-receipts"
source = "lisp/agent/verification_actions_receipt.lisp"
subject = "verify:daemon"

allowed_prefixes = (
    "gh run list",
    "gh run view",
    "curl -I -L",
    "grep -R",
)

required_checks = {"CI", "Security", "Golden", "Verification Graph"}

required_fields = {"headSha", "status", "conclusion", "url"}

required_kinds = {
    "workflow-summary",
    "head-sha",
    "job-detail",
    "pages-health",
    "forbidden-event",
}

required_rules = {
    "actions-receipt:required-fields",
    "actions-receipt:allowed-tools",
    "actions-receipt:evidence-linked",
}
