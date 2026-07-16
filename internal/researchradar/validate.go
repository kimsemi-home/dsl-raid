package researchradar

import (
	"errors"
	"regexp"
	"slices"
)

var idPattern = regexp.MustCompile(`^[a-z0-9][a-z0-9-]{2,63}$`)

func Validate(manifest Manifest) error {
	if manifest.SchemaVersion != ManifestSchema || !datePattern.MatchString(manifest.ReviewedOn) ||
		len(manifest.Sources) != len(requiredSources()) ||
		!slices.Equal(manifest.Checks, requiredChecks()) {
		return errors.New("research radar header is invalid")
	}
	seen := map[string]bool{}
	for _, source := range manifest.Sources {
		if seen[source.ID] || validateSource(source, manifest.ReviewedOn) != nil {
			return errors.New("research radar source is invalid")
		}
		seen[source.ID] = true
	}
	for id, expected := range requiredSources() {
		found := findSource(manifest.Sources, id)
		if found == nil || found.Decision != expected.decision || found.Target != expected.target {
			return errors.New("research radar required decision is missing")
		}
	}
	if manifest.AggregateHash != aggregateHash(manifest) {
		return errors.New("research radar aggregate hash is invalid")
	}
	return nil
}

func findSource(sources []Source, id string) *Source {
	for index := range sources {
		if sources[index].ID == id {
			return &sources[index]
		}
	}
	return nil
}

func requiredChecks() []string {
	return []string{"aggregator-primary-separated", "decisions-actionable", "official-constraints-win", "revalidation-bounded", "source-urls-https", "targets-explicit"}
}
