(in-package #:dslraid.agent)

(defun write-ci-lisp-job (out)
  (write-ci-lines
   out
   '("  lisp:"
     "    name: Lisp authoring"
     "    runs-on: ubuntu-latest"
     "    steps:"
     "      - uses: actions/checkout@v6"
     "      - name: Install SBCL"
     "        run: bash scripts/install-sbcl.sh"
     "      - name: Verify quality owns generated backends"
     "        run: bash scripts/verificationqualitygen.sh check"
     "")))

(defun write-ci-dslraid-job (out)
  (write-ci-lines
   out
   '("  dslraid:"
     "    name: DSLRaid quality"
     "    runs-on: ubuntu-latest"
     "    timeout-minutes: 30"
     "    steps:"
     "      - uses: actions/checkout@v6"
     "      - name: Install SBCL"
     "        timeout-minutes: 20"
     "        run: bash scripts/install-sbcl.sh"
     "      - uses: dtolnay/rust-toolchain@stable"
     "      - uses: Swatinem/rust-cache@v2"
     "      - name: Run unified quality gate"
     "        run: cargo run -p dslraid-cli -- quality"
     "")))
