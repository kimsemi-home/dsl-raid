(in-package #:dslraid.lang)

(defun keyword-tail (form)
  (case (dsl-form-head form)
    ((:state :event :transition :guard :action)
     (when (primary-id-present-p form)
       (rest (dsl-form-args form))))
    (:defined-at (dsl-form-args form))
    (otherwise nil)))

(defun allowed-keywords (head)
  (case head
    (:state '(:kind :initial :terminal :terminal-semantics :defined-at :tags))
    (:event '(:kind))
    (:transition '(:from :to :on :guards :actions :requires :defined-at :tags))
    (:guard '(:kind :expression :input :defined-at :tags))
    (:action '(:kind :command :emits :expression :defined-at :tags))
    (:defined-at '(:uri :start-line :end-line))
    (otherwise nil)))

(defun malformed-keyword-tail-p (tail)
  (or (oddp (length tail))
      (loop for pair on tail by #'cddr
            thereis (not (keywordp (first pair))))))
