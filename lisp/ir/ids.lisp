(in-package #:dslraid.ir)

(defun kebab-name (symbol-or-string)
  (let ((raw (string-downcase
              (etypecase symbol-or-string
                (symbol (symbol-name symbol-or-string))
                (string symbol-or-string)))))
    (string-trim
     "-"
     (with-output-to-string (out)
       (let ((previous-separator nil))
         (loop for ch across raw
               do (if (alphanumericp ch)
                      (progn
                        (write-char ch out)
                        (setf previous-separator nil))
                      (unless previous-separator
                        (write-char #\- out)
                        (setf previous-separator t)))))))))

(defun semantic-id (kind name)
  (format nil "~(~A~):~A" kind (kebab-name name)))

(defun fsm-local-name (fsm-or-id)
  (let ((id (etypecase fsm-or-id
              (fsm (fsm-id fsm-or-id))
              (string fsm-or-id))))
    (if (and (>= (length id) 4) (string= id "fsm:" :end1 4))
        (subseq id 4)
        id)))

(defun state-subject (fsm-or-id state-id)
  (format nil "state:~A.~A" (fsm-local-name fsm-or-id) (kebab-name state-id)))

(defun transition-subject (fsm-or-id transition-id)
  (format nil "transition:~A.~A" (fsm-local-name fsm-or-id) (kebab-name transition-id)))
