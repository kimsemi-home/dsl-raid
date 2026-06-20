(in-package #:dslraid.agent)

(defun write-pages-deploy-job (out)
  (write-pages-lines
   out
   '("  deploy:"
     "    name: Deploy demo"
     "    runs-on: ubuntu-latest"
     "    environment:"
     "      name: github-pages"
     "    steps:"
     "      - uses: actions/checkout@v6"
     "      - uses: dtolnay/rust-toolchain@stable"
     "      - uses: actions/setup-node@v6"
     "        with:"
     "          node-version: 24"
     "          cache: npm"
     "          cache-dependency-path: apps/viewer/package-lock.json"
     "      - name: Render example assets"
     "        run: |"
     "          cargo run -p dslraid-cli -- demo package \\"
     "            examples/runscope/runscope.raid.json \\"
     "            --out apps/viewer/public/examples \\"
     "            --trace examples/runscope/run-001.trace.json"
     "      - name: Build viewer"
     "        working-directory: apps/viewer"
     "        run: |"
     "          npm ci"
     "          npm run build"
     "      - uses: actions/upload-pages-artifact@v5"
     "        with:"
     "          path: apps/viewer/dist"
     "      - uses: actions/deploy-pages@v5")))
