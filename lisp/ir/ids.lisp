(in-package #:dslraid.ir)

(defun kebab-name (symbol-or-string)
  (let ((raw (string-downcase
              (etypecase symbol-or-string
                (symbol (symbol-name symbol-or-string))
                (string symbol-or-string)))))
    (string-trim
     "_"
     (with-output-to-string (out)
       (let ((previous-separator nil))
         (loop with index = 0
               while (< index (length raw))
               do (multiple-value-setq (index previous-separator)
                    (write-canonical-id-part raw index out
                                             previous-separator))))))))

(defun write-canonical-id-part (raw index out previous-separator)
  (let ((ch (char raw index)))
    (cond
      ((arrow-at-p raw index)
       (write-string "_to_" out)
       (values (+ index 2) nil))
      ((alphanumericp ch)
       (write-char ch out)
       (values (1+ index) nil))
      (previous-separator
       (values (1+ index) t))
      (t
       (write-char #\_ out)
       (values (1+ index) t)))))

(defun arrow-at-p (raw index)
  (and (char= (char raw index) #\-)
       (< (1+ index) (length raw))
       (char= (char raw (1+ index)) #\>)))


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
