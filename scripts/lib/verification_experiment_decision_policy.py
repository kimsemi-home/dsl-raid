generated_by = "scripts/verificationexperimentdecisiongen.sh"
profile = "hypothesis-to-act-closure"
source = "lisp/agent/verification_experiment_decision.lisp"
subject = "verify:daemon"

allowed_decisions = {"drop", "promote", "repeat"}
checked_status = {"checked", "promoted"}

required_decisions = {
    "experiment-decision:bootstrap-sequence",
    "experiment-decision:run-manifest",
}

required_rules = {
    "experiment-decision:act-matches",
    "experiment-decision:known-experiment",
    "experiment-decision:promote-after-check",
}
