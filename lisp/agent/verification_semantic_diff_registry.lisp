(in-package #:dslraid.agent)

(defparameter *verification-semantic-core-diffs*
  (append *verification-semantic-foundation-diffs*
          *verification-semantic-governance-diffs*
          *verification-semantic-runtime-diffs*
          *verification-semantic-ssot-diffs*))

(defparameter *verification-semantic-diffs*
  (append *verification-semantic-core-diffs*
          *verification-semantic-tail-diffs*))
