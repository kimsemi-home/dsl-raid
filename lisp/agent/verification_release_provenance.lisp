(in-package #:dslraid.agent)

(defparameter *verification-release-provenance*
  '(("release-provenance:tag-trigger" "tag-trigger"
     ".github/workflows/release.yml" ("tags:" "- 'v*'")
     ("docs/generated/verification-github-actions.json")
     "Release promotion starts from an explicit version tag.")
    ("release-provenance:generated-check" "generated-workflow-check"
     ".github/workflows/release.yml" ("bash scripts/releasegen.sh check")
     ("docs/generated/verification-conformance.json")
     "Generated release workflow must prove it is fresh.")
    ("release-provenance:build-test" "release-build-test"
     ".github/workflows/release.yml" ("cargo test --workspace")
     ("docs/generated/verification-conformance.json")
     "Release promotion requires workspace test evidence.")
    ("release-provenance:publish-permission" "publish-permission"
     ".github/workflows/release.yml" ("permissions:" "contents: write")
     ("docs/generated/verification-access-policy.json")
     "Release publishing requires explicit write permission.")
    ("release-provenance:create-release" "publish-command"
     ".github/workflows/release.yml" ("GH_TOKEN" "gh release create")
     ("docs/generated/verification-security-audit.json")
     "Release command uses GitHub-provided token only inside the workflow.")))

(defparameter *verification-release-provenance-rules*
  '(("release-provenance:tag-bound" "Release publication must be tag-bound.")
    ("release-provenance:fresh-generated-workflow" "Generated release workflow must be fresh.")
    ("release-provenance:evidence-linked" "Each release gate links generated evidence.")))

(defun emit-verification-release-provenance-json (&optional stream)
  "Emit release promotion provenance gates."
  (let ((json (with-output-to-string (out)
                (write-verification-release-provenance out))))
    (if stream (write-string json stream) json)))

(defun write-verification-release-provenance (out)
  (format out "{~%")
  (format out "  \"manifest_version\": \"0.1.0\",~%")
  (format out "  \"generated_by\": \"scripts/verificationreleaseprovenancegen.sh\",~%")
  (format out "  \"subject\": \"~A\",~%" (getf (verification-graph) :id))
  (format out "  \"source\": \"lisp/agent/verification_release_provenance.lisp\",~%")
  (format out "  \"release_profile\": \"tag-bound-generated-release\",~%")
  (write-release-provenance-gates out)
  (format out ",~%")
  (write-release-provenance-rules out)
  (format out "~%}~%"))

(defun write-release-provenance-gates (out)
  (format out "  \"gates\": [~%")
  (loop for row in *verification-release-provenance*
        for first = t then nil
        do (unless first (format out ",~%"))
           (write-release-provenance-gate out row))
  (format out "~%  ]"))

(defun write-release-provenance-gate (out row)
  (destructuring-bind (id gate workflow requires evidence meaning) row
    (format out "    {\"id\": \"~A\", \"gate\": \"~A\", " id gate)
    (format out "\"workflow\": \"~A\", " workflow)
    (write-authority-list out "requires" requires)
    (format out ", ")
    (write-authority-list out "evidence" evidence)
    (format out ", \"status\": \"required\", \"meaning\": \"~A\"}" meaning)))

(defun write-release-provenance-rules (out)
  (format out "  \"closure_rules\": [~%")
  (loop for row in *verification-release-provenance-rules*
        for first = t then nil
        do (unless first (format out ",~%"))
           (destructuring-bind (id meaning) row
             (format out "    {\"id\": \"~A\", \"meaning\": \"~A\", " id meaning)
             (format out "\"check\": \"scripts/verificationreleaseprovenancegen.sh check\"}")))
  (format out "~%  ]"))
