generated_by = "scripts/verificationknowledgegen.sh"
profile = "error-to-knowledge"
subject = "verify:daemon"
source = "lisp/agent/verification_knowledge_conversion.lisp"

required_steps = [
    ("knowledge-conversion:incompleteness", "incompleteness-visible"),
    ("knowledge-conversion:failure", "failure-signaled"),
    ("knowledge-conversion:evidence", "evidence-captured"),
    ("knowledge-conversion:root-cause", "root-cause-mapped"),
    ("knowledge-conversion:debt", "debt-recorded"),
    ("knowledge-conversion:incident-learning", "incident-learned"),
    ("knowledge-conversion:learning-loop", "knowledge-updated"),
    ("knowledge-conversion:version-propagated", "version-propagated"),
    ("knowledge-conversion:revalidated", "runtime-revalidated"),
]

required_rules = {
    "knowledge-conversion:visible",
    "knowledge-conversion:owned",
    "knowledge-conversion:closed",
}
