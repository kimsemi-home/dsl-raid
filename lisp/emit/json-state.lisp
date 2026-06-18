(in-package #:dslraid.emit)

(defun write-state-json (state out level)
  (indent out level)
  (format out "{")
  (format out "\"id\": ~A, \"kind\": ~A"
          (json-string (state-id state))
          (json-string (state-kind state)))
  (when (state-initial-p state)
    (format out ", \"initial\": true"))
  (when (state-terminal-p state)
    (format out ", \"terminal\": true"))
  (when (state-terminal-semantics state)
    (format out ", \"terminal_semantics\": ~A"
            (json-string (state-terminal-semantics state))))
  (write-defined-at-property (state-defined-at state) out)
  (format out "}"))
