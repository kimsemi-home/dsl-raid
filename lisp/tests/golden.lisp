(in-package #:dslraid)

(defun run-golden-smoke ()
  (let ((fsms (runscope-fsms)))
    (dolist (fsm fsms)
      (assert (null (validate-fsm fsm))))
    (emit-project-json "runscope" "RunScope" fsms)))
