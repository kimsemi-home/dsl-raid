(in-package #:dslraid.lang)

(defparameter *keyword-diagnostic-codes*
  '((:key :unknown-keyword
     :code "LANG014"
     :severity "error"
     :scope "FSM authoring keyword arguments"
     :summary "Authoring form uses an unsupported keyword.")
    (:key :malformed-keyword-list
     :code "LANG015"
     :severity "error"
     :scope "FSM authoring keyword arguments"
     :summary "Authoring keyword arguments must be key/value pairs.")))
