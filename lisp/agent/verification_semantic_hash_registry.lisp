(in-package #:dslraid.agent)

(defparameter *verification-semantic-core-hashes*
  (append *verification-semantic-foundation-hashes*
          *verification-semantic-governance-hashes*
          *verification-semantic-runtime-hashes*
          *verification-semantic-ssot-hashes*))

(defparameter *verification-semantic-hashes*
  (append *verification-semantic-core-hashes*
          *verification-semantic-tail-hashes*
          *verification-semantic-tail-extra-hashes*))
