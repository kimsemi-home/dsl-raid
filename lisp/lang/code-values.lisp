(in-package #:dslraid.lang)

(defparameter *value-diagnostic-codes*
  '((:key :invalid-boolean-value
     :code "LANG016"
     :severity "error"
     :scope "FSM authoring values"
     :summary "Boolean authoring keyword values must be true or false.")))
