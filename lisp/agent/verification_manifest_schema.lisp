(in-package #:dslraid.agent)

(defparameter *verification-manifest-schema-json*
  (concatenate
   'string
   "{\"$schema\":\"https://json-schema.org/draft/2020-12/schema\","
   "\"$id\":\"https://dslraid.dev/schemas/dslraid-verification-manifest.schema.json\","
   "\"title\":\"DSLRaid Verification Manifest\",\"oneOf\":["
   (verification-manifest-schema-variants-json)
   "],"
   (verification-manifest-schema-defs-json)
   "}"))

(defun emit-verification-manifest-schema-json (&optional stream)
  (if stream (write-string *verification-manifest-schema-json* stream) *verification-manifest-schema-json*))
