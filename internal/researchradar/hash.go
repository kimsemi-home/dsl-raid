package researchradar

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
)

func digest(value []byte) string {
	hash := sha256.Sum256(value)
	return hex.EncodeToString(hash[:])
}

func aggregateHash(manifest Manifest) string {
	copy := manifest
	copy.AggregateHash = ""
	body, _ := json.Marshal(copy)
	return digest(body)
}

func sealReport(report Report) Report {
	report.ReportHash = ""
	body, _ := json.Marshal(report)
	report.ReportHash = digest(body)
	return report
}
