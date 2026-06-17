(in-package #:dslraid.lang)

(defparameter *value-diagnostic-codes*
  '((:key :invalid-boolean-value
     :code "LANG016"
     :severity "error"
     :scope "FSM authoring values"
     :summary "Boolean authoring keyword values must be true or false.")
    (:key :invalid-collection-value
     :code "LANG017"
     :severity "error"
     :scope "FSM authoring values"
     :summary "Collection authoring keyword values must be lists.")
    (:key :invalid-enum-value
     :code "LANG018"
     :severity "error"
     :scope "FSM authoring values"
     :summary "Enum authoring keyword values must match Core IR enums.")))
