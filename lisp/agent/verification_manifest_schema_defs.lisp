(in-package #:dslraid.agent)

(defun verification-manifest-schema-defs-json ()
  (concatenate
   'string
   (verification-manifest-schema-core-defs-json)
   (verification-manifest-schema-quality-defs-json)
   (verification-manifest-schema-governance-defs-json)
   (verification-manifest-schema-ontology-defs-json)
   (verification-manifest-schema-runtime-defs-json)
   (verification-manifest-schema-tail-defs-json)))
