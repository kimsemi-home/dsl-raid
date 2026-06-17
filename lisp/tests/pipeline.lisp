(in-package #:dslraid)

(defun run-language-boundary-smoke ()
  (let ((boundaries (dslraid.lang::language-boundary-catalog)))
    (assert (equal (mapcar #'first boundaries)
                   '("Expansion" "Conformance" "Projection")))
    (assert (search "Rust, Go" (fifth (third boundaries))))
    boundaries))

(defun run-language-contract-smoke ()
  (let ((contracts (dslraid.lang:language-contract-catalog)))
    (assert (member "Rust source can be generated runtime output, not authoring truth."
                    contracts
                    :test #'equal))
    contracts))
