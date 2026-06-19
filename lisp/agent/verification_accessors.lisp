(in-package #:dslraid.agent)

(defun verification-graph ()
  (copy-tree *verification-graph*))

(defun verification-nodes ()
  (getf (verification-graph) :nodes))

(defun verification-field (node key)
  (getf node key))

(defun verification-backends ()
  (copy-tree *verification-backends*))

(defun verification-id (node)
  (verification-field node :id))
