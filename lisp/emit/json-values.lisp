(in-package #:dslraid.emit)

(defun write-json-array (items out level writer)
  (loop for item in items
        for last = (eq item (car (last items)))
        do (progn
             (funcall writer item out level)
             (unless last (format out ","))
             (format out "~%")))
  (indent out (1- level))
  (format out "]"))

(defun json-field (out level key value comma-p)
  (indent out level)
  (format out "\"~A\": ~A~:[~;,~]~%" key (json-string value) comma-p))

(defun json-string-array (values)
  (format nil "[~{~A~^, ~}]" (mapcar #'json-string values)))

(defun json-string (value)
  (format nil "\"~A\"" (escape-json (princ-to-string value))))

(defun escape-json (value)
  (with-output-to-string (out)
    (loop for ch across value
          do (write-escaped-char ch out))))

(defun write-escaped-char (ch out)
  (case ch
    (#\\ (write-string "\\\\" out))
    (#\" (write-string "\\\"" out))
    (#\Newline (write-string "\\n" out))
    (#\Return (write-string "\\r" out))
    (#\Tab (write-string "\\t" out))
    (otherwise (write-char ch out))))

(defun indent (out level)
  (loop repeat (* level 2)
        do (write-char #\Space out)))
