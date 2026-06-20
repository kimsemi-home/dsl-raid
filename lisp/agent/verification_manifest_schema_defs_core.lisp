(in-package #:dslraid.agent)

(defun verification-manifest-schema-core-defs-json ()
  (concatenate
   'string
   "\"$defs\":{\"path\":{\"type\":\"string\",\"minLength\":1},"
   "\"semantic_ref\":{\"type\":\"string\",\"pattern\":\"^[a-z][a-z0-9_\\\\-]*:[a-z][a-z0-9_.\\\\-]*$\"},"
   "\"semver\":{\"type\":\"string\",\"pattern\":\"^[0-9]+\\\\.[0-9]+\\\\.[0-9]+$\"},"
   "\"strings\":{\"type\":\"array\",\"minItems\":1,\"items\":{\"type\":\"string\",\"minLength\":1}},"
   "\"rule\":{\"type\":\"object\",\"required\":[\"id\",\"meaning\",\"check\"],"
   "\"additionalProperties\":false,\"properties\":{\"id\":{\"type\":\"string\",\"minLength\":1},"
   "\"meaning\":{\"type\":\"string\",\"minLength\":1},\"check\":{\"type\":\"string\",\"pattern\":\"^scripts/.+ check$\"}}},"
   "\"status_rule\":{\"type\":\"object\",\"required\":[\"id\",\"meaning\",\"check\",\"status\"],"
   "\"additionalProperties\":false,\"properties\":{\"id\":{\"type\":\"string\",\"minLength\":1},"
   "\"meaning\":{\"type\":\"string\",\"minLength\":1},\"check\":{\"type\":\"string\",\"pattern\":\"^scripts/.+ check$\"},"
   "\"status\":{\"const\":\"required\"}}},"
   "\"axis\":{\"type\":\"object\",\"required\":[\"axis\",\"backends\"],"
   "\"additionalProperties\":false,\"properties\":{\"axis\":{\"type\":\"string\",\"minLength\":1},"
   "\"backends\":{\"$ref\":\"#/$defs/strings\"}}},"
   "\"pdca_step\":{\"type\":\"object\",\"required\":[\"phase\",\"evidence\",\"artifact\"],"
   "\"additionalProperties\":false,\"properties\":{\"phase\":{\"enum\":[\"plan\",\"do\",\"check\",\"act\"]},"
   "\"evidence\":{\"type\":\"string\",\"minLength\":1},\"artifact\":{\"$ref\":\"#/$defs/path\"}}},"))
