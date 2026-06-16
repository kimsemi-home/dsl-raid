(in-package #:dslraid.lang)

(defparameter *language-diagnostics*
  (append *authoring-diagnostic-codes*
          *transition-diagnostic-codes*
          *identifier-diagnostic-codes*))

(defun language-code (key)
  (getf (or (find key *language-diagnostics*
                  :key (lambda (entry) (getf entry :key)))
            (error "Unknown language diagnostic key ~A" key))
        :code))

(defun language-diagnostic-catalog ()
  (sort (copy-list *language-diagnostics*)
        #'string<
        :key (lambda (entry) (getf entry :code))))
