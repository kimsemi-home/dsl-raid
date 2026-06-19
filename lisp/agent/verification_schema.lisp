(in-package #:dslraid.agent)

(defparameter *verification-evidence-schema-json*
  (concatenate
   'string
   "{\"$schema\":\"https://json-schema.org/draft/2020-12/schema\","
   "\"$id\":\"https://dslraid.dev/schemas/dslraid-verification-evidence.schema.json\","
   "\"title\":\"DSLRaid Verification Evidence\",\"type\":\"object\","
   "\"required\":[\"$schema\",\"schema_version\",\"generated_by\",\"subject\","
   "\"ssot\",\"form\",\"ontology_chain\",\"generated_backends\","
   "\"verification_nodes\",\"pdca\"],\"additionalProperties\":false,"
   "\"properties\":{\"$schema\":{\"const\":\"schemas/dslraid-verification-evidence.schema.json\"},"
   "\"schema_version\":{\"$ref\":\"#/$defs/semver\"},"
   "\"generated_by\":{\"const\":\"scripts/verificationevidencegen.sh\"},"
   "\"subject\":{\"$ref\":\"#/$defs/semantic_ref\"},"
   "\"ssot\":{\"$ref\":\"#/$defs/path\"},"
   "\"form\":{\"type\":\"string\",\"minLength\":1},"
   "\"ontology_chain\":{\"type\":\"array\",\"prefixItems\":[{\"const\":\"ontology\"},"
   "{\"const\":\"executable-ssot\"},{\"const\":\"verification-graph\"},"
   "{\"const\":\"codegen\"}],\"minItems\":4,\"maxItems\":4},"
   "\"generated_backends\":{\"type\":\"array\",\"minItems\":1,"
   "\"items\":{\"$ref\":\"#/$defs/backend\"}},"
   "\"verification_nodes\":{\"type\":\"array\",\"minItems\":1,"
   "\"items\":{\"$ref\":\"#/$defs/node\"}},"
   "\"pdca\":{\"type\":\"array\",\"prefixItems\":[{\"const\":\"plan\"},"
   "{\"const\":\"do\"},{\"const\":\"check\"},{\"const\":\"act\"}],"
   "\"minItems\":4,\"maxItems\":4}},"
   "\"$defs\":{\"backend\":{\"type\":\"object\","
   "\"required\":[\"backend\",\"output\",\"generator\",\"check\"],"
   "\"additionalProperties\":false,\"properties\":{\"backend\":{\"type\":\"string\","
   "\"minLength\":1},\"output\":{\"$ref\":\"#/$defs/path\"},"
   "\"generator\":{\"$ref\":\"#/$defs/path\"},"
   "\"check\":{\"type\":\"string\",\"pattern\":\"^scripts/.+ check$\"}}},"
   "\"node\":{\"type\":\"object\",\"required\":[\"id\",\"commands\",\"evidence\"],"
   "\"additionalProperties\":false,\"properties\":{\"id\":{\"type\":\"string\","
   "\"pattern\":\"^[a-z][a-z0-9\\\\-]*$\"},"
   "\"commands\":{\"type\":\"integer\",\"minimum\":1},"
   "\"evidence\":{\"type\":\"string\",\"minLength\":1}}},"
   "\"path\":{\"type\":\"string\",\"minLength\":1},"
   "\"semver\":{\"type\":\"string\",\"pattern\":\"^[0-9]+\\\\.[0-9]+\\\\.[0-9]+$\"},"
   "\"semantic_ref\":{\"type\":\"string\","
   "\"pattern\":\"^[a-z][a-z0-9_\\\\-]*:[a-z][a-z0-9_.\\\\-]*$\"}}}"))

(defun emit-verification-evidence-schema-json (&optional stream)
  "Emit JSON Schema for verification evidence."
  (let ((json *verification-evidence-schema-json*))
    (if stream
        (write-string json stream)
        json)))
