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
   (:module "passes"
    :serial t
    :components
    ((:file "normalize")
     (:file "validation")))
   (:module "emit"
    :serial t
    :components
    ((:file "json")))))
