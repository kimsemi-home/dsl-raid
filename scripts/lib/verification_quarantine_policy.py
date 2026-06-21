generated_by = "scripts/verificationquarantinegen.sh"
profile = "promotion-blocking-isolation"
source = "lisp/agent/verification.lisp"
subject = "verify:daemon"

allowed_status = {"closed", "isolated", "released"}
allowed_subject_kinds = {"generated-artifact", "tool-execution"}

required_blocks = {
    "artifact-commit",
    "automatic-approval",
    "confidence-increase",
}

required_bundles = {
    "quarantine:suspicious-generated-output",
    "quarantine:tool-behavior",
}

required_rules = {
    "quarantine:blocks-approval",
    "quarantine:blocks-commit",
    "quarantine:blocks-confidence",
    "quarantine:evidence-linked",
}
