(defpackage #:dslraid.ir
  (:use #:cl)
  (:export
   #:defined-at #:make-defined-at #:defined-at-uri
   #:defined-at-start-line #:defined-at-end-line
   #:state #:make-state #:state-id #:state-kind #:state-initial-p
   #:state-terminal-p #:state-terminal-semantics #:state-defined-at
   #:event #:make-event #:event-id #:event-kind
   #:transition #:make-transition #:transition-id #:transition-from
   #:transition-to #:transition-on #:transition-guards #:transition-actions
   #:transition-requires #:transition-defined-at
   #:fsm #:make-fsm #:fsm-id #:fsm-name #:fsm-states #:fsm-events
   #:fsm-transitions #:fsm-guards #:fsm-actions #:fsm-defined-at #:fsm-tags
   #:semantic-id #:state-subject #:transition-subject #:fsm-display-name))

(defpackage #:dslraid.lang
  (:use #:cl #:dslraid.ir)
  (:export
   #:parse-fsm-form
   #:validate-fsm-ast
   #:expand-fsm-ast))

(defpackage #:dslraid.dsl
  (:use #:cl #:dslraid.ir)
  (:export
   #:fsm
   #:defdsl-fsm
   #:build-fsm))

(defpackage #:dslraid.expansion
  (:use #:cl #:dslraid.ir)
  (:export
   #:normalize-fsm))

(defpackage #:dslraid.conformance
  (:use #:cl #:dslraid.ir)
  (:export
   #:validate-fsm))

(defpackage #:dslraid.emit
  (:use #:cl #:dslraid.ir)
  (:export
   #:emit-fsm-json
   #:emit-project-json
   #:emit-language-pipeline-markdown
   #:emit-language-diagnostics-markdown))

(defpackage #:dslraid
  (:use #:cl)
  (:import-from #:dslraid.lang
                #:parse-fsm-form
                #:validate-fsm-ast
                #:expand-fsm-ast)
  (:import-from #:dslraid.dsl #:fsm #:defdsl-fsm #:build-fsm)
  (:import-from #:dslraid.expansion #:normalize-fsm)
  (:import-from #:dslraid.conformance #:validate-fsm)
  (:import-from #:dslraid.emit
                #:emit-fsm-json
                #:emit-project-json
                #:emit-language-pipeline-markdown
                #:emit-language-diagnostics-markdown)
  (:export
   #:fsm
   #:defdsl-fsm
   #:build-fsm
   #:parse-fsm-form
   #:validate-fsm-ast
   #:expand-fsm-ast
   #:normalize-fsm
   #:validate-fsm
   #:emit-fsm-json
   #:emit-project-json
   #:emit-language-pipeline-markdown
   #:emit-language-diagnostics-markdown
   #:runscope-fsms))
