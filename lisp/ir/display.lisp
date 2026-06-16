(in-package #:dslraid.ir)

(defun display-name (symbol-or-string)
  (let ((raw (kebab-name symbol-or-string)))
    (with-output-to-string (out)
      (loop for start = 0 then (1+ end)
            for end = (position #\_ raw :start start)
            do (write-string (title-part (subseq raw start end)) out)
            while end))))

(defun fsm-display-name (symbol-or-string)
  (let ((base (display-name symbol-or-string)))
    (if (suffix-equal-p base "FSM")
        (concatenate 'string (subseq base 0 (- (length base) 3)) "FSM")
        (concatenate 'string base "FSM"))))

(defun title-part (part)
  (if (zerop (length part))
      ""
      (concatenate 'string
                   (string-upcase (subseq part 0 1))
                   (subseq part 1))))

(defun suffix-equal-p (value suffix)
  (let ((start (- (length value) (length suffix))))
    (and (>= start 0)
         (string-equal value suffix :start1 start))))
