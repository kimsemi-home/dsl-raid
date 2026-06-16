(in-package #:dslraid.lang)

(defparameter *identifier-diagnostic-codes*
  '((:key :missing-state-id
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
