generated_by = "scripts/verificationfailuregen.sh"
profile = "operational-breakage"
source = "lisp/agent/verification.lisp"
subject = "verify:daemon"

domains = {
    "ontology",
    "confidence",
    "reviewer",
    "control-plane",
    "lease",
    "translation",
    "evidence-quality",
    "feedback",
}
severities = {"error", "warning", "info"}

required_conditions = [
    "failure:ontology-context",
    "failure:confidence-self",
    "failure:reviewer-isolation",
    "failure:control-plane-manifest",
    "failure:lease-missing",
    "failure:translation-loss",
    "failure:evidence-quality-stale",
    "failure:feedback-open",
]

required_rules = {
    "failure:blocks-explicit",
    "failure:evidence-linked",
    "failure:owner-required",
}
