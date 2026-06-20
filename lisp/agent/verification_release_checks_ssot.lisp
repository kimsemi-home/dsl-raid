(in-package #:dslraid.agent)

(defparameter *release-check-ssot-groups*
  '(("ssot" "versioned SSOT, diagnosis, and debt checks"
     ("bash scripts/verificationgenesisgen.sh check"
      "bash scripts/verificationmetamodelgen.sh check"
      "bash scripts/verificationstewardgen.sh check"
      "bash scripts/verificationrevalidationgen.sh check"
      "bash scripts/verificationcoldstartgen.sh check"
      "bash scripts/verificationseparationgen.sh check"
      "bash scripts/verificationevidencebeforechangegen.sh check"
      "bash scripts/verificationversionedssotgen.sh check"
      "bash scripts/verificationcontextmapgen.sh check"
      "bash scripts/verificationhistoricalgen.sh check"
      "bash scripts/verificationtransitiongen.sh check"
      "bash scripts/verificationssotdefectgen.sh check"
      "bash scripts/verificationrootcausegen.sh check"
      "bash scripts/verificationdebuggergen.sh check"
      "bash scripts/verificationpruninggen.sh check"
      "bash scripts/verificationsecuritygen.sh check"
      "bash scripts/verificationfailuregen.sh check"
      "bash scripts/verificationfailurerecoverygen.sh check"
      "bash scripts/verificationdebtgen.sh check"
      "bash scripts/verificationincompletegen.sh check"))))
