generated_by = "scripts/verificationlearninggen.sh"
profile = "reality-to-revalidation"
source = "lisp/agent/verification_learning_loop.lisp"
subject = "verify:daemon"

required_stages = [
    ("stage:reality", "runtime-event", "observation"),
    ("stage:observation", "observation", "evidence"),
    ("stage:evidence", "evidence", "interpretation"),
    ("stage:interpretation", "interpretation", "rulebook"),
    ("stage:rulebook", "rulebook", "design"),
    ("stage:design", "design", "codegen"),
    ("stage:codegen", "codegen", "revalidation"),
]

required_rules = {
    "learning-loop:ordered",
    "learning-loop:evidence-linked",
    "learning-loop:update-required",
    "learning-loop:owner-required",
}

statuses = {"open", "closed", "revalidating"}
