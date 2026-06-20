generated_by = "scripts/verificationruntimegen.sh"
profile = "design-coverage-overlay"
source = "lisp/agent/verification_runtime_trace.lisp"
subject = "verify:daemon"

coverage_statuses = {
    "covered",
    "uncovered",
    "failed",
}

required_rules = {
    "runtime-trace:design-known",
    "runtime-trace:coverage-linked",
    "runtime-trace:no-contradiction",
}
