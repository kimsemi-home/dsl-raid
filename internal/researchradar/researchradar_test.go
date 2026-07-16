package researchradar

import (
	"encoding/json"
	"os"
	"path/filepath"
	"runtime"
	"testing"
)

func TestFixtureAndReportValidate(t *testing.T) {
	manifest, err := Read(fixturePath(t))
	if err != nil {
		t.Fatal(err)
	}
	report, err := BuildReport(manifest)
	if err != nil {
		t.Fatal(err)
	}
	if report.SourceCount != 6 || report.AdoptCount != 1 || report.AdaptCount != 4 || report.DeferCount != 1 || report.ReportHash == "" {
		t.Fatalf("unexpected report: %#v", report)
	}
}

func TestDeviceFlowCannotBeAdoptedForYouTubeUpload(t *testing.T) {
	manifest, err := Read(fixturePath(t))
	if err != nil {
		t.Fatal(err)
	}
	for index := range manifest.Sources {
		if manifest.Sources[index].ID == "google-device-flow-scope-limit" {
			manifest.Sources[index].Decision = "adopt"
		}
	}
	manifest.AggregateHash = aggregateHash(manifest)
	if err := Validate(manifest); err == nil {
		t.Fatal("accepted Google device flow for the unsupported youtube.upload scope")
	}
}

func TestResealedReportCannotClaimMoreAdoptions(t *testing.T) {
	manifest, err := Read(fixturePath(t))
	if err != nil {
		t.Fatal(err)
	}
	report, err := BuildReport(manifest)
	if err != nil {
		t.Fatal(err)
	}
	report.AdoptCount++
	report.AdaptCount--
	report = sealReport(report)
	if err := ValidateReport(report, manifest); err == nil {
		t.Fatal("accepted a hash-valid report with false decision counts")
	}
}

func TestFixtureAggregateHashIsCurrent(t *testing.T) {
	manifest, err := ReadWithoutValidation(fixturePath(t))
	if err != nil {
		t.Fatal(err)
	}
	if actual := aggregateHash(manifest); actual != manifest.AggregateHash {
		t.Fatalf("research radar aggregate hash = %s", actual)
	}
}

func ReadWithoutValidation(path string) (Manifest, error) {
	body, err := os.ReadFile(path)
	if err != nil {
		return Manifest{}, err
	}
	var manifest Manifest
	return manifest, json.Unmarshal(body, &manifest)
}

func fixturePath(t *testing.T) string {
	t.Helper()
	_, file, _, _ := runtime.Caller(0)
	return filepath.Join(filepath.Dir(file), "..", "..", "examples", "source-radar", "research-decisions.json")
}
