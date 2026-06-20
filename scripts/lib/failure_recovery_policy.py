generated_by = "scripts/verificationfailurerecoverygen.sh"
profile = "failure-to-learning"
source = "lisp/agent/verification_failure_recovery.lisp"
subject = "verify:daemon"

statuses = {
    "release-blocked-until-revalidated",
    "learning-open",
    "closed",
}

required_recoveries = [
    "failure-recovery:control-plane-manifest",
    "failure-recovery:evidence-quality-stale",
    "failure-recovery:feedback-open",
]

required_rules = {
    "failure-recovery:known-failure",
    "failure-recovery:response-matches",
    "failure-recovery:learning-linked",
    "failure-recovery:evidence-linked",
}
