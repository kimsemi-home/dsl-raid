(in-package #:dslraid.agent)

(defun emit-verification-pages-yaml (&optional stream)
  (let ((yaml (with-output-to-string (out) (write-verification-pages out))))
    (if stream (write-string yaml stream) yaml)))

(defun write-verification-pages (out)
  (write-pages-header out)
  (write-pages-deploy-job out))
