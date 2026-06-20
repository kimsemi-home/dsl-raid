generated_by = "scripts/verificationrootcausegen.sh"
profile = "candidate-validation"
source = "lisp/agent/verification.lisp"
subject = "verify:daemon"

statuses = {"candidate-set", "confirmed", "rejected", "closed"}
confidence_levels = {"low", "medium", "high"}

required_cases = [
    ("root-cause:ssot-defect-drill", "candidate-set"),
]

required_rules = {
    "root-cause:candidates-first",
    "root-cause:validation-required",
    "root-cause:confidence-capped",
}
