(in-package #:dslraid.agent)

(defun write-ci-viewer-job (out)
  (write-ci-lines
   out
   '("  viewer:"
     "    name: Viewer"
     "    runs-on: ubuntu-latest"
     "    steps:"
     "      - uses: actions/checkout@v6"
     "      - uses: actions/setup-node@v6"
     "        with:"
     "          node-version: 24"
     "          cache: npm"
     "          cache-dependency-path: apps/viewer/package-lock.json"
     "      - name: Install viewer dependencies"
     "        working-directory: apps/viewer"
     "        run: npm ci"
     "      - name: Lint viewer"
     "        working-directory: apps/viewer"
     "        run: npm run lint"
     "      - name: Test viewer"
     "        working-directory: apps/viewer"
     "        run: npm test"
     "      - name: Build viewer"
     "        working-directory: apps/viewer"
     "        run: npm run build")))
