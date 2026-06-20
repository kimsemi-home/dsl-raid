(in-package #:dslraid.agent)

(defparameter *verification-github-actions*
  '(("github-actions:ci" ".github/workflows/ci.yml" "generated-ci" t
     "scripts/cigen.sh" ("contents: read") ("docs/generated/verification-conformance.json")
     "CI workflow is generated from the Lisp SSOT.")
    ("github-actions:golden" ".github/workflows/golden.yml" "generated-golden" t
     "scripts/goldengen.sh" ("contents: read") ("tests/golden/verification-graph.generated.json")
     "Golden workflow is generated from the Lisp SSOT.")
    ("github-actions:security" ".github/workflows/security.yml" "generated-security" t
     "scripts/securityworkflowgen.sh" ("contents: read" "security-events: write")
     ("docs/generated/verification-security-audit.json")
     "Security workflow is generated from the Lisp SSOT.")
    ("github-actions:pages" ".github/workflows/pages.yml" "generated-pages" t
     "scripts/pagesworkflowgen.sh" ("contents: read" "pages: write" "id-token: write")
     ("docs/generated/verification-sidecar.json")
     "Pages workflow is generated from the Lisp SSOT.")
    ("github-actions:verification" ".github/workflows/verification.yml" "generated-graph" t
     "scripts/workflowgen.sh" ("contents: read") ("docs/generated/verification-backend-parity.json")
     "Verification Graph workflow is generated from the Lisp SSOT.")
    ("github-actions:release" ".github/workflows/release.yml" "generated-release" t
     "scripts/releasegen.sh" ("contents: write") ("docs/generated/verification-conformance.json")
     "Release workflow is generated and checked before publishing.")))

(defparameter *verification-github-actions-rules*
  '(("github-actions:generated-header" "Generated workflows must name their generator.")
    ("github-actions:least-permission" "Every workflow declares explicit permissions.")
    ("github-actions:evidence-linked" "Every workflow links generated evidence.")))

(defun emit-verification-github-actions-json (&optional stream)
  "Emit GitHub Actions suite ownership and permission evidence."
  (let ((json (with-output-to-string (out)
                (write-verification-github-actions out))))
    (if stream (write-string json stream) json)))

(defun write-verification-github-actions (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationactionsgen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_github_actions.lisp\",~%")
  (format out "  \"workflow_suite_profile\": \"generated-plus-curated-guards\",~%")
  (write-github-action-workflows out)
  (format out ",~%")
  (write-github-action-rules out)
  (format out "~%}~%"))

(defun write-github-action-workflows (out)
  (format out "  \"workflows\": [~%")
  (loop for row in *verification-github-actions*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-github-action-workflow out row))
  (format out "~%  ]"))

(defun write-github-action-workflow (out row)
  (destructuring-bind (id workflow role generated generator permissions evidence meaning) row
    (format out "    {\"id\": \"~A\", \"workflow\": \"~A\", " id workflow)
    (format out "\"role\": \"~A\", \"generated\": ~A, " role (if generated "true" "false"))
    (format out "\"generator\": \"~A\", " generator)
    (write-authority-list out "permissions" permissions)
    (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"meaning\": \"~A\"}" meaning)))

(defun write-github-action-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-github-actions-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationactionsgen.sh check\"}")))
  (format out "~%  ]"))
