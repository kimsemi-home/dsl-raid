(in-package #:dslraid)

(defparameter *runtime-fsm*
  (fsm runtime
    (:defined-at :uri "lisp/runtime/runscope.lisp" :start-line 12 :end-line 44)
    (:state idle :initial t)
    (:state starting)
    (:state running)
    (:state completed :terminal t :terminal-semantics "success")
    (:event start-requested)
    (:transition idle->starting :from idle :to starting :on start-requested)
    (:transition starting->running :from starting :to running)
    (:transition running->completed :from running :to completed)))

(defun run-golden-smoke ()
  (let ((diagnostics (validate-fsm *runtime-fsm*)))
    (assert (null diagnostics))
    (emit-fsm-json *runtime-fsm*)))
