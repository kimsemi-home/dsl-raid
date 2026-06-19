(in-package #:dslraid.agent)

(defparameter *verification-backends*
  (append *verification-core-backends* *verification-tail-backends*))
