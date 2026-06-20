(in-package #:dslraid.agent)

(defun verification-manifest-schema-quality-defs-json ()
  (concatenate
   'string
   "\"evidence_ops_record\":{\"type\":\"object\",\"required\":[\"id\",\"operation\",\"claim\",\"subject\",\"evidence\",\"updates\",\"authority_effect\",\"owner\",\"meaning\"],\"additionalProperties\":false,\"properties\":{\"id\":{\"$ref\":\"#/$defs/semantic_ref\"},\"operation\":{\"enum\":[\"ci\",\"quality-gate\",\"release\",\"experiment\",\"incident\"]},\"claim\":{\"enum\":[\"evidence-generator\",\"conformance-claim\",\"pdca-experiment\",\"knowledge-update\"]},\"subject\":{\"$ref\":\"#/$defs/semantic_ref\"},\"evidence\":{\"$ref\":\"#/$defs/strings\"},\"updates\":{\"$ref\":\"#/$defs/strings\"},\"authority_effect\":{\"enum\":[\"evidence-only\",\"review-required\",\"release-gated\",\"learning-loop\"]},\"owner\":{\"$ref\":\"#/$defs/semantic_ref\"},\"meaning\":{\"type\":\"string\",\"minLength\":1}}},"
   "\"loss_entry\":{\"type\":\"object\",\"required\":[\"id\",\"source\",\"target\","
   "\"loss_level\",\"meaning\",\"evidence\",\"policy\"],\"additionalProperties\":false,"
   "\"properties\":{\"id\":{\"$ref\":\"#/$defs/semantic_ref\"},"
   "\"source\":{\"$ref\":\"#/$defs/semantic_ref\"},\"target\":{\"$ref\":\"#/$defs/path\"},"
   "\"loss_level\":{\"enum\":[\"L1\",\"L2\",\"L3\"]},"
   "\"meaning\":{\"type\":\"string\",\"minLength\":1},\"evidence\":{\"$ref\":\"#/$defs/path\"},"
   "\"policy\":{\"type\":\"string\",\"minLength\":1}}},"
   "\"semantic_hash\":{\"type\":\"object\",\"required\":[\"id\",\"source\",\"fields\",\"meaning\",\"hash\"],\"additionalProperties\":false,\"properties\":{\"id\":{\"$ref\":\"#/$defs/semantic_ref\"},\"source\":{\"$ref\":\"#/$defs/path\"},\"fields\":{\"$ref\":\"#/$defs/strings\"},\"meaning\":{\"type\":\"string\",\"minLength\":1},\"hash\":{\"type\":\"string\",\"pattern\":\"^[a-f0-9]{64}$\"}}},"
   "\"semantic_diff\":{\"type\":\"object\",\"required\":[\"id\",\"hash_id\",\"summary\",\"base_hash\",\"head_hash\",\"status\",\"evidence\"],\"additionalProperties\":false,\"properties\":{\"id\":{\"$ref\":\"#/$defs/semantic_ref\"},\"hash_id\":{\"$ref\":\"#/$defs/semantic_ref\"},\"summary\":{\"type\":\"string\",\"minLength\":1},\"base_hash\":{\"type\":\"string\",\"pattern\":\"^[a-f0-9]{64}$\"},\"head_hash\":{\"type\":\"string\",\"pattern\":\"^[a-f0-9]{64}$\"},\"status\":{\"enum\":[\"unchanged\",\"changed\",\"blocked\"]},\"evidence\":{\"$ref\":\"#/$defs/path\"}}},"))
