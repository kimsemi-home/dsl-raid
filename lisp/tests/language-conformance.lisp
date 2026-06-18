(in-package #:dslraid)

(defun run-build-fsm-conformance-smoke ()
  (let ((blocked nil))
    (handler-case
        (build-fsm 'bad-demo
                   '((:state idle)
                     (:transition broken :from idle :to missing)))
      (error ()
        (setf blocked t)))
    (assert blocked)))
