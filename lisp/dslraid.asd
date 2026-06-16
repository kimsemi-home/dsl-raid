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
     (:file "ids")
     (:file "display")))
   (:module "lang"
    :serial t
    :components
    ((:file "ast")
     (:file "codes")
     (:file "diagnostic")
     (:file "forms")
     (:file "duplicates")
     (:file "conformance")
     (:file "expand-items")
     (:file "expand")))
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
     (:file "json-source")
     (:file "json-fsm")
     (:file "json-derived")
     (:file "json")
     (:file "markdown")
     (:file "diagnostics")))
   (:module "runtime"
    :serial t
    :components
    ((:file "runscope")))))
