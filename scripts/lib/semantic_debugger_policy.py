generated_by = "scripts/verificationdebuggergen.sh"
profile = "question-oriented"
source = "lisp/agent/verification.lisp"
subject = "verify:daemon"

risk_levels = {
    "low",
    "medium",
    "high",
}

escalations = {
    "none",
    "review",
    "authority-gate",
}

required_rules = {
    "semantic-debugger:inputs-linked",
    "semantic-debugger:missing-evidence-declared",
    "semantic-debugger:no-final-cause",
}
