generated_by = "scripts/verificationevidencegraphgen.sh"
profile = "linked-evidence-to-authority"
source = "lisp/agent/verification_evidence_graph.lisp"
subject = "verify:daemon"

required_relations = {
    "observes",
    "interprets",
    "supports",
    "gates",
    "updates",
}

required_rules = {
    "evidence-graph:no-orphans",
    "evidence-graph:evidence-exists",
    "evidence-graph:feedback-closes",
}
