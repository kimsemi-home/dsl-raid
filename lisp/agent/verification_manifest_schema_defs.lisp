(in-package #:dslraid.agent)

(defun verification-manifest-schema-defs-json ()
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
   "\"evidence\":{\"type\":\"string\",\"minLength\":1},\"artifact\":{\"$ref\":\"#/$defs/path\"}}},"
   "\"loss_entry\":{\"type\":\"object\",\"required\":[\"id\",\"source\",\"target\","
   "\"loss_level\",\"meaning\",\"evidence\",\"policy\"],\"additionalProperties\":false,"
   "\"properties\":{\"id\":{\"$ref\":\"#/$defs/semantic_ref\"},"
   "\"source\":{\"$ref\":\"#/$defs/semantic_ref\"},\"target\":{\"$ref\":\"#/$defs/path\"},"
   "\"loss_level\":{\"enum\":[\"L1\",\"L2\",\"L3\"]},"
   "\"meaning\":{\"type\":\"string\",\"minLength\":1},\"evidence\":{\"$ref\":\"#/$defs/path\"},"
   "\"policy\":{\"type\":\"string\",\"minLength\":1}}},"
   "\"semantic_hash\":{\"type\":\"object\",\"required\":[\"id\",\"source\",\"fields\",\"meaning\",\"hash\"],\"additionalProperties\":false,\"properties\":{\"id\":{\"$ref\":\"#/$defs/semantic_ref\"},\"source\":{\"$ref\":\"#/$defs/path\"},\"fields\":{\"$ref\":\"#/$defs/strings\"},\"meaning\":{\"type\":\"string\",\"minLength\":1},\"hash\":{\"type\":\"string\",\"pattern\":\"^[a-f0-9]{64}$\"}}},"
   "\"semantic_diff\":{\"type\":\"object\",\"required\":[\"id\",\"hash_id\",\"summary\",\"base_hash\",\"head_hash\",\"status\",\"evidence\"],\"additionalProperties\":false,\"properties\":{\"id\":{\"$ref\":\"#/$defs/semantic_ref\"},\"hash_id\":{\"$ref\":\"#/$defs/semantic_ref\"},\"summary\":{\"type\":\"string\",\"minLength\":1},\"base_hash\":{\"type\":\"string\",\"pattern\":\"^[a-f0-9]{64}$\"},\"head_hash\":{\"type\":\"string\",\"pattern\":\"^[a-f0-9]{64}$\"},\"status\":{\"enum\":[\"unchanged\",\"changed\",\"blocked\"]},\"evidence\":{\"$ref\":\"#/$defs/path\"}}},"
   "\"authority_decision\":{\"type\":\"object\",\"required\":[\"id\",\"scope\",\"decision\",\"approved_by\",\"requires\",\"evidence\",\"meaning\"],\"additionalProperties\":false,\"properties\":{\"id\":{\"$ref\":\"#/$defs/semantic_ref\"},\"scope\":{\"enum\":[\"routine\",\"release\",\"security\",\"audit\",\"ontology\",\"incident\"]},\"decision\":{\"enum\":[\"approved\",\"escalated\",\"rejected\"]},\"approved_by\":{\"$ref\":\"#/$defs/semantic_ref\"},\"requires\":{\"$ref\":\"#/$defs/strings\"},\"evidence\":{\"$ref\":\"#/$defs/strings\"},\"meaning\":{\"type\":\"string\",\"minLength\":1}}}}"))
