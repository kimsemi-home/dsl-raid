generated_by = "scripts/verificationexecutablegen.sh"
profile = "machine-readable-ssot"
subject = "verify:daemon"

required_kinds = [
    "dsl",
    "specification",
    "ontology",
    "policy",
    "schema",
    "contract",
    "manifest",
    "ir",
    "verification-rule",
    "migration-rule",
    "translation-manifest",
    "evidence-policy",
]

required_rules = {
    "executable-knowledge:no-prose-ssot",
    "executable-knowledge:command-backed",
    "executable-knowledge:artifact-linked",
}
