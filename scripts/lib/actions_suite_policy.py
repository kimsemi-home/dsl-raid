expected_workflows = {
    ".github/workflows/ci.yml",
    ".github/workflows/golden.yml",
    ".github/workflows/security.yml",
    ".github/workflows/pages.yml",
    ".github/workflows/verification.yml",
    ".github/workflows/release.yml",
}

forbidden_tokens = [
    "pull_request_target:",
    "write-all",
    "contents: write-all",
]
