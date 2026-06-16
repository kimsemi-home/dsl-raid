(asdf:defsystem #:dslraid
  :description "Common Lisp DSL layer for DSLRaid typed executable IR."
  :license "Apache-2.0"
  :serial t
  :components
  ((:file "packages")
   (:module "ir"
    :serial t
    :components
    ((:file "model")
     (:file "ids")))
   (:module "dsl"
    :serial t
    :components
    ((:file "expand")
     (:file "syntax")))
   (:module "expansion"
    :serial t
    :components
    ((:file "normalize")))
   (:module "conformance"
    :serial t
    :components
    ((:file "validation")))
   (:module "emit"
    :serial t
    :components
    ((:file "json-values")
     (:file "json-fsm")
     (:file "json")))))
