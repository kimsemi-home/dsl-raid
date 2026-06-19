(in-package #:dslraid.agent)

(defun verification-manifest-schema-variants-json ()
  (concatenate
   'string
   (verification-manifest-schema-core-variants-json)
   (verification-manifest-schema-tail-variants-json)))
