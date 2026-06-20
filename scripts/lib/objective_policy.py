required_kinds = {
    "privacy-exclusion", "autonomous-merge", "source-shape",
    "backend-projection", "actions-hardening", "release-pipeline",
    "codegen-chain", "pdca-learning", "evidence-governance",
    "actions-receipt", "query-lazy-surface", "learning-loop",
    "local-precommit",
}

gate_evidence = {
    "gate:privacy": ["docs/generated/verification-privacy.json"],
    "gate:merge-readiness": ["docs/generated/verification-merge-readiness.json"],
    "gate:source-shape": ["docs/generated/verification-source-shape.json"],
    "gate:precommit": ["docs/generated/verification-precommit-closure.json"],
    "gate:backend-parity": ["docs/generated/verification-backend-parity.json"],
    "gate:github-actions": ["docs/generated/verification-github-actions.json"],
    "gate:release": ["docs/generated/verification-release-provenance.json"],
    "gate:codegen": ["docs/generated/verification-codegen.json"],
    "gate:pdca": ["docs/generated/verification-pdca.json"],
    "gate:evidence-governance": ["docs/generated/verification-pruning.json"],
    "gate:learning-loop": ["docs/generated/verification-learning-loop.json"],
    "gate:actions-receipt": ["docs/generated/verification-actions-receipt.json"],
    "gate:query-surface": ["docs/generated/verification-query-surface.json"],
}

required_backends = (
    "github-actions", "gitlab-ci", "local-makefile", "bazel",
    "source-shape", "query-surface", "evidence-graph",
    "public-projection",
)

required_axes = (
    "code", "docs", "schemas", "tests", "conformance",
    "github-actions", "release-pipelines",
)
