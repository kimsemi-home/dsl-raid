(in-package #:dslraid.lang)

(defparameter *language-diagnostics*
  '((:key :duplicate-state
     :code "LANG001"
     :severity "error"
     :scope "state authoring forms"
     :summary "Duplicate state form.")
    (:key :duplicate-event
     :code "LANG002"
     :severity "error"
     :scope "event authoring forms"
     :summary "Duplicate event form.")
    (:key :duplicate-transition
     :code "LANG003"
     :severity "error"
     :scope "transition authoring forms"
     :summary "Duplicate transition form.")
    (:key :unknown-form
     :code "LANG004"
     :severity "error"
     :scope "FSM authoring forms"
     :summary "Unknown FSM authoring form.")
    (:key :unknown-transition-from
     :code "LANG005"
     :severity "error"
     :scope "transition authoring forms"
     :summary "Transition :from references an undeclared state.")
    (:key :unknown-transition-to
     :code "LANG006"
     :severity "error"
     :scope "transition authoring forms"
     :summary "Transition :to references an undeclared state.")
    (:key :unknown-transition-event
     :code "LANG007"
     :severity "error"
     :scope "transition authoring forms"
     :summary "Transition :on references an undeclared event.")
    (:key :missing-transition-from
     :code "LANG008"
     :severity "error"
     :scope "transition authoring forms"
     :summary "Transition is missing required :from state.")
    (:key :missing-transition-to
     :code "LANG009"
     :severity "error"
     :scope "transition authoring forms"
     :summary "Transition is missing required :to state.")
    (:key :missing-state-id
     :code "LANG010"
     :severity "error"
     :scope "state authoring forms"
     :summary "State form is missing its id.")
    (:key :missing-event-id
     :code "LANG011"
     :severity "error"
     :scope "event authoring forms"
     :summary "Event form is missing its id.")
    (:key :missing-transition-id
     :code "LANG012"
     :severity "error"
     :scope "transition authoring forms"
     :summary "Transition form is missing its id.")))

(defun language-code (key)
  (getf (or (find key *language-diagnostics*
                  :key (lambda (entry) (getf entry :key)))
            (error "Unknown language diagnostic key ~A" key))
        :code))

(defun language-diagnostic-catalog ()
  (copy-list *language-diagnostics*))
