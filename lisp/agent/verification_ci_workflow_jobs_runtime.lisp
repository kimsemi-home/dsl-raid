(in-package #:dslraid.agent)

(defun write-ci-rust-job (out)
  (write-ci-lines
   out
   '("  rust:"
     "    name: Rust"
     "    runs-on: ubuntu-latest"
     "    steps:"
     "      - uses: actions/checkout@v6"
     "      - uses: dtolnay/rust-toolchain@stable"
     "        with:"
     "          components: rustfmt, clippy"
     "      - uses: Swatinem/rust-cache@v2"
     "      - name: Check formatting"
     "        run: cargo fmt --all -- --check"
     "      - name: Clippy"
     "        run: cargo clippy --workspace --all-targets -- -D warnings"
     "      - name: Test"
     "        run: cargo test --workspace"
     "")))

(defun write-ci-go-job (out)
  (write-ci-lines
   out
   '("  go:"
     "    name: Go lint"
     "    runs-on: ubuntu-latest"
     "    steps:"
     "      - uses: actions/checkout@v6"
     "      - uses: actions/setup-go@v6"
     "        with:"
     "          go-version: 1.26.5"
     "          cache: true"
     "          cache-dependency-path: |"
     "            go.mod"
     "            generated/go.mod"
     "      - name: Run golangci-lint"
     "        env:"
     "          GOLANGCI_LINT_VERSION: v2.12.2"
     "        run: scripts/go-lint.sh"
     "")))

(defun write-ci-source-shape-job (out)
  (write-ci-lines
   out
   '("  source-shape:"
     "    name: Source shape"
     "    runs-on: ubuntu-latest"
     "    steps:"
     "      - uses: actions/checkout@v6"
     "      - name: Enforce source line budget"
     "        run: bash scripts/check-source-lines.sh"
     "      - name: Check generated roadmap index"
     "        run: bash scripts/roadmapgen.sh check"
     "      - name: Check generated docs index"
     "        run: bash scripts/gendocindex.sh check"
     "")))
