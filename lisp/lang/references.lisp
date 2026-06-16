(in-package #:dslraid.lang)

(defun transition-reference-diagnostics (ast)
  "Return authoring diagnostics for transition refs before IR expansion."
  (let ((states (declared-ids ast :state "state"))
        (events (declared-ids ast :event "event"))
        (diagnostics '()))
    (dolist (form (fsm-ast-forms ast))
      (when (eq (dsl-form-head form) :transition)
        (setf diagnostics
              (append-transition-reference-diagnostics
               ast form states events diagnostics))))
    (nreverse diagnostics)))

(defun append-transition-reference-diagnostics (ast form states events diagnostics)
  (destructuring-bind (id &key from to on &allow-other-keys)
      (dsl-form-args form)
    (declare (ignore id))
    (let ((next (append-state-reference
                 ast form from :unknown-transition-from diagnostics "from" states)))
      (setf next (append-state-reference
                  ast form to :unknown-transition-to next "to" states))
      (if on
          (append-event-reference ast form on events next)
          next))))

(defun append-state-reference (ast form value key diagnostics slot states)
  (if (or (null value) (gethash (semantic-id "state" value) states))
      diagnostics
      (cons (unknown-reference-diagnostic ast form key slot value "state")
            diagnostics)))

(defun append-event-reference (ast form value events diagnostics)
  (if (gethash (semantic-id "event" value) events)
      diagnostics
      (cons (unknown-reference-diagnostic ast form :unknown-transition-event
                                          "event" value "event")
            diagnostics)))

(defun declared-ids (ast head label)
  (let ((ids (make-hash-table :test 'equal)))
    (dolist (form (fsm-ast-forms ast))
      (when (eq (dsl-form-head form) head)
        (let ((key (form-key label form)))
          (when key
            (setf (gethash key ids) t)))))
    ids))

(defun unknown-reference-diagnostic (ast form key slot value kind)
  (language-diagnostic
   (language-code key)
   :error
   (ast-form-subject ast form)
   (unknown-reference-message slot value kind)
   (format nil "Declare ~A \"~A\" before referencing it." kind value)))

(defun unknown-reference-message (slot value kind)
  (format nil "Transition :~A references unknown ~A \"~A\"."
          slot kind value))
