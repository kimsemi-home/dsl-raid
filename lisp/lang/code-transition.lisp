(in-package #:dslraid.lang)

(defparameter *transition-diagnostic-codes*
  '((:key :unknown-transition-from
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
     :summary "Transition is missing required :to state.")))
