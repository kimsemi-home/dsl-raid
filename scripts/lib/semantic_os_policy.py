generated_by = "scripts/verificationsemanticosgen.sh"
profile = "meaning-operating-system"
subject = "verify:daemon"

required_roles = {
    "firmware",
    "kernel",
    "filesystem",
    "userland",
    "driver",
    "log",
    "scheduler",
    "court",
}

required_rules = {
    "semantic-os:file-backed",
    "semantic-os:authority-gated",
    "semantic-os:generated-runtime",
}
