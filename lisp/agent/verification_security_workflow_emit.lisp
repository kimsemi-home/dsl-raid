(in-package #:dslraid.agent)

(defun emit-verification-security-yaml (&optional stream)
  (let ((yaml (with-output-to-string (out) (write-verification-security out))))
    (if stream (write-string yaml stream) yaml)))

(defun write-verification-security (out)
  (write-security-header out)
  (write-security-audit-job out)
  (write-security-secrets-job out))
