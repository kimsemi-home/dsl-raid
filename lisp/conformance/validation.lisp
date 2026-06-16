(in-package #:dslraid.conformance)

(defun validate-fsm (fsm)
  "Return language conformance diagnostics for an expanded FSM."
  (let* ((states (fsm-states fsm))
         (state-ids (mapcar #'state-id states))
         (events (mapcar #'event-id (fsm-events fsm)))
         (initials (remove-if-not #'state-initial-p states))
         (terminals (mapcar #'state-id (remove-if-not #'state-terminal-p states)))
         (diagnostics '()))
    (labels ((emit (code severity subject message suggestion)
               (push (list :code code
                           :severity severity
                           :subject subject
                           :message message
                           :suggestion suggestion)
                     diagnostics)))
      (when (null states)
        (emit "FSM006" :error (fsm-id fsm)
              "FSM must define at least one state."
              "Add one or more (:state ...) forms."))
      (unless (= (length initials) 1)
        (emit "FSM007" :error (fsm-id fsm)
              (format nil "FSM has ~D initial states." (length initials))
              "Mark exactly one state with :initial t."))
      (dolist (transition (fsm-transitions fsm))
        (validate-transition fsm transition state-ids events terminals #'emit))
      (nreverse diagnostics))))

(defun validate-transition (fsm transition state-ids events terminals emit)
  (let ((subject (transition-subject fsm (transition-id transition))))
    (unless (member (transition-from transition) state-ids :test #'string=)
      (funcall emit "FSM008" :error subject
               "transition.from does not resolve inside the FSM."
               "Use a state declared by (:state ...)."))
    (unless (member (transition-to transition) state-ids :test #'string=)
      (funcall emit "FSM009" :error subject
               "transition.to does not resolve inside the FSM."
               "Use a state declared by (:state ...)."))
    (when (and (transition-on transition)
               (not (member (transition-on transition) events :test #'string=)))
      (funcall emit "FSM010" :error subject
               "transition.on references an unknown event."
               "Add a matching (:event ...)."))
    (when (member (transition-from transition) terminals :test #'string=)
      (funcall emit "FSM011" :error subject
               "terminal state has an outgoing transition."
               "Remove the outgoing transition or remove :terminal t."))))
