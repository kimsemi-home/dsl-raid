generated_by = "scripts/verificationprecommitgen.sh"
profile = "local-quality-gate"
source = "lisp/agent/verification_precommit_closure.lisp"
subject = "verify:daemon"

required_kinds = {
    "go-lint",
    "rust-format",
    "rust-clippy",
    "rust-test",
    "viewer-lint",
    "viewer-test",
    "viewer-build",
    "quality",
    "diff-check",
}

required_rules = {
    "precommit:hook-installed",
    "precommit:commands-covered",
    "precommit:quality-required",
    "precommit:private-safe",
}
