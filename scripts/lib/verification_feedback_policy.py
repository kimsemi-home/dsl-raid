generated_by = "scripts/verificationfeedbackgen.sh"
profile = "knowledge-closure-loop"
source = "lisp/agent/verification.lisp"
subject = "verify:daemon"

allowed_status = {"closed", "open", "revalidating"}

required_closures = {
    "feedback:review-overload",
    "feedback:stale-generated-output",
}

required_rules = {
    "feedback:evidence-linked",
    "feedback:owner-required",
    "feedback:revalidation-required",
    "feedback:update-required",
}

allowed_update_prefixes = ("policy:", "spec:")
