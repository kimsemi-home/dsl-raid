generated_by = "scripts/verificationsidecargen.sh"
profile = "independent-verifier"
source = "lisp/agent/verification.lisp"
subject = "verify:daemon"

required_rules = {
    "sidecar:evidence-generated",
    "sidecar:independent",
    "sidecar:producer-separated",
}

required_receipts = {
    "sidecar:confidence",
    "sidecar:generated-workflow",
    "sidecar:semantic-hash",
}
