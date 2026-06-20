graph_nodes = ("lint", "unit-test", "integration-test", "conformance", "release-check")

expected_surfaces = {
    "github-actions": (".github/workflows/verification.yml", graph_nodes),
    "gitlab-ci": (".gitlab-ci.yml", graph_nodes),
    "local-makefile": ("Makefile", graph_nodes),
    "bazel": ("BUILD.bazel", graph_nodes),
    "release-check-provider": ("scripts/releasecheck", ("release-check",)),
}
