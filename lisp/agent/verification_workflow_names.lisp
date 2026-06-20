(in-package #:dslraid.agent)

(defun verification-command-name (id index command)
  (cond
    ((string= command "bash scripts/releasecheckgen.sh check")
     "Check release-check provider scripts")
    ((release-check-provider-command-p command)
     (format nil "Run release-check provider ~A"
             (release-check-provider-name command)))
    (t (format nil "Run ~A command ~D" id index))))

(defun release-check-provider-command-p (command)
  (and (verification-prefix-p "bash scripts/releasecheck/" command)
       (verification-suffix-p ".sh" command)))

(defun release-check-provider-name (command)
  (let* ((prefix "bash scripts/releasecheck/")
         (start (length prefix))
         (end (- (length command) 3)))
    (subseq command start end)))

(defun verification-prefix-p (prefix value)
  (let ((size (length prefix)))
    (and (<= size (length value))
         (string= prefix (subseq value 0 size)))))

(defun verification-suffix-p (suffix value)
  (let ((start (- (length value) (length suffix))))
    (and (<= 0 start)
         (string= suffix (subseq value start)))))
