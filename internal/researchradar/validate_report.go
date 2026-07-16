package researchradar

import (
	"errors"
	"slices"
	"sort"
)

func ValidateReport(report Report, manifest Manifest) error {
	if err := Validate(manifest); err != nil {
		return err
	}
	targetSet := map[string]bool{}
	adopt, adapt, deferred := 0, 0, 0
	for _, source := range manifest.Sources {
		targetSet[source.Target] = true
		switch source.Decision {
		case "adopt":
			adopt++
		case "adapt":
			adapt++
		case "defer":
			deferred++
		}
	}
	targets := make([]string, 0, len(targetSet))
	for target := range targetSet {
		targets = append(targets, target)
	}
	sort.Strings(targets)
	sealed := sealReport(report)
	if report.SchemaVersion != ReportSchema || report.SourceCount != len(manifest.Sources) ||
		report.AdoptCount != adopt || report.AdaptCount != adapt || report.DeferCount != deferred ||
		!slices.Equal(report.Targets, targets) || report.ExternalNetwork || report.ExternalWrites ||
		report.AggregateHash != manifest.AggregateHash || !slices.Equal(report.Checks, manifest.Checks) ||
		report.ReportHash == "" || report.ReportHash != sealed.ReportHash {
		return errors.New("research radar report is invalid")
	}
	return nil
}
