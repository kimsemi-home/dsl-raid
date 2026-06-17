(in-package #:dslraid)

(defun run-language-enum-value-smoke ()
  (let* ((ast (parse-fsm-form
               'enum-value-demo
               '((:state idle :kind actor)
                 (:state done :terminal-semantics winner)
                 (:event happened :kind cosmic))))
         (diagnostics (validate-fsm-ast ast)))
    (assert-language-codes diagnostics
                           '("LANG018" "LANG018" "LANG018"))
    diagnostics))

(defun run-language-enum-normalization-smoke ()
  (let ((fsm (build-fsm 'enum-normalization
                        '((:state idle :initial t)
                          (:state done :kind compound
                           :terminal t :terminal-semantics retriable-failed)
                          (:event tick :kind timer)
                          (:transition finish :from idle :to done :on tick)))))
    (assert (string= (dslraid.ir:state-kind
                      (second (dslraid.ir:fsm-states fsm)))
                     "compound"))
    (assert (string= (dslraid.ir:event-kind
                      (first (dslraid.ir:fsm-events fsm)))
                     "timer"))
    fsm))
