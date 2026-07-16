package researchradar

import "sort"

func BuildReport(manifest Manifest) (Report, error) {
	if err := Validate(manifest); err != nil {
		return Report{}, err
	}
	targetSet := map[string]bool{}
	report := Report{
		SchemaVersion: ReportSchema, SourceCount: len(manifest.Sources),
		ExternalNetwork: false, ExternalWrites: false,
		AggregateHash: manifest.AggregateHash, Checks: append([]string{}, manifest.Checks...),
	}
	for _, source := range manifest.Sources {
		targetSet[source.Target] = true
		switch source.Decision {
		case "adopt":
			report.AdoptCount++
		case "adapt":
			report.AdaptCount++
		case "defer":
			report.DeferCount++
		}
	}
	for target := range targetSet {
		report.Targets = append(report.Targets, target)
	}
	sort.Strings(report.Targets)
	report = sealReport(report)
	if err := ValidateReport(report, manifest); err != nil {
		return Report{}, err
	}
	return report, nil
}
