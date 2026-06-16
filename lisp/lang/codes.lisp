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
     :summary "Unknown FSM authoring form.")))

(defun language-code (key)
  (getf (or (find key *language-diagnostics*
                  :key (lambda (entry) (getf entry :key)))
            (error "Unknown language diagnostic key ~A" key))
        :code))

(defun language-diagnostic-catalog ()
  (copy-list *language-diagnostics*))
