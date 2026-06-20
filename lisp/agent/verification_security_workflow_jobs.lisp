(in-package #:dslraid.agent)

(defun write-security-audit-job (out)
  (write-security-lines
   out
   '("  audit:"
     "    name: Rust dependency audit"
     "    runs-on: ubuntu-latest"
     "    steps:"
     "      - uses: actions/checkout@v6"
     "      - uses: dtolnay/rust-toolchain@stable"
     "      - name: Install cargo-deny"
     "        env:"
     "          CARGO_HTTP_MULTIPLEXING: \"false\""
     "        run: |"
     "          set -euo pipefail"
     "          for attempt in 1 2 3; do"
     "            if cargo install cargo-deny --locked; then"
     "              exit 0"
     "            fi"
     "            sleep \"$((attempt * 5))\""
     "          done"
     "          exit 1"
     "      - name: Run cargo-deny"
     "        run: cargo deny check"
     "")))

(defun write-security-secrets-job (out)
  (write-security-lines
   out
   '("  secrets:"
     "    name: Secret scan"
     "    runs-on: ubuntu-latest"
     "    steps:"
     "      - uses: actions/checkout@v6"
     "        with:"
     "          fetch-depth: 0"
     "      - uses: gitleaks/gitleaks-action@v3"
     "        env:"
     "          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}")))
