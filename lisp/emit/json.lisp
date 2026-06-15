(in-package #:dslraid.emit)

(defun emit-fsm-json (fsm &optional (stream nil))
  "Emit deterministic JSON for a single FSM object."
  (let ((json (with-output-to-string (out)
                (write-fsm-json fsm out 0))))
    (if stream
        (write-string json stream)
        json)))

(defun emit-project-json (project-id project-name fsms &optional (stream nil))
  "Emit a minimal canonical Core IR document containing FSMs."
  (let ((json (with-output-to-string (out)
                (format out "{~%")
                (json-field out 1 "ir_version" "0.1.0" t)
                (indent out 1)
                (format out "\"project\": {\"id\": ~A, \"name\": ~A},~%"
                        (json-string project-id)
                        (json-string project-name))
                (indent out 1)
                (format out "\"fsms\": [~%")
                (loop for fsm in fsms
                      for last = (eq fsm (car (last fsms)))
                      do (progn
                           (write-fsm-json fsm out 2)
                           (unless last (format out ","))
                           (format out "~%")))
                (indent out 1)
                (format out "]~%")
                (format out "}~%"))))
    (if stream
        (write-string json stream)
        json)))

(defun write-fsm-json (fsm out level)
  (indent out level)
  (format out "{~%")
  (json-field out (1+ level) "id" (fsm-id fsm) t)
  (json-field out (1+ level) "name" (fsm-name fsm) t)
  (indent out (1+ level))
  (format out "\"states\": [~%")
  (write-json-array (fsm-states fsm) out (+ level 2) #'write-state-json)
  (format out ",~%")
  (indent out (1+ level))
  (format out "\"events\": [~%")
  (write-json-array (fsm-events fsm) out (+ level 2) #'write-event-json)
  (format out ",~%")
  (indent out (1+ level))
  (format out "\"transitions\": [~%")
  (write-json-array (fsm-transitions fsm) out (+ level 2) #'write-transition-json)
  (format out "~%")
  (indent out level)
  (format out "}"))

(defun write-state-json (state out level)
  (indent out level)
  (format out "{")
  (format out "\"id\": ~A, \"kind\": ~A" (json-string (state-id state)) (json-string (state-kind state)))
  (when (state-initial-p state)
    (format out ", \"initial\": true"))
  (when (state-terminal-p state)
    (format out ", \"terminal\": true"))
  (when (state-terminal-semantics state)
    (format out ", \"terminal_semantics\": ~A" (json-string (state-terminal-semantics state))))
  (format out "}"))

(defun write-event-json (event out level)
  (indent out level)
  (format out "{\"id\": ~A, \"kind\": ~A}" (json-string (event-id event)) (json-string (event-kind event))))

(defun write-transition-json (transition out level)
  (indent out level)
  (format out "{")
  (format out "\"id\": ~A, \"from\": ~A, \"to\": ~A"
          (json-string (transition-id transition))
          (json-string (transition-from transition))
          (json-string (transition-to transition)))
  (when (transition-on transition)
    (format out ", \"on\": ~A" (json-string (transition-on transition))))
  (when (transition-guards transition)
    (format out ", \"guards\": ~A" (json-string-array (transition-guards transition))))
  (when (transition-actions transition)
    (format out ", \"actions\": ~A" (json-string-array (transition-actions transition))))
  (when (transition-requires transition)
    (format out ", \"requires\": ~A" (json-string-array (transition-requires transition))))
  (format out "}"))

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
          do (case ch
               (#\\ (write-string "\\\\" out))
               (#\" (write-string "\\\"" out))
               (#\Newline (write-string "\\n" out))
               (#\Return (write-string "\\r" out))
               (#\Tab (write-string "\\t" out))
               (otherwise (write-char ch out))))))

(defun indent (out level)
  (loop repeat (* level 2)
        do (write-char #\Space out)))
