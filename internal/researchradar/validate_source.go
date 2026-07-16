package researchradar

import (
	"errors"
	"net/url"
	"regexp"
	"time"
)

var datePattern = regexp.MustCompile(`^\d{4}-\d{2}-\d{2}$`)

func validateSource(source Source, reviewedOn string) error {
	if !idPattern.MatchString(source.ID) || source.Topic == "" || source.Action == "" || source.Rationale == "" ||
		(source.Decision != "adopt" && source.Decision != "adapt" && source.Decision != "defer") ||
		(source.Target != "dsl-raid" && source.Target != "myhome-ledger" && source.Target != "myhome-shorts-factory") ||
		len(source.EvidenceURLs) == 0 || len(source.EvidenceURLs) > 4 || len(source.Guardrails) == 0 || len(source.Guardrails) > 6 {
		return errors.New("source fields are incomplete")
	}
	if validateHTTPS(source.DiscoveryURL) != nil || validateHTTPS(source.PrimaryURL) != nil || source.DiscoveryURL == source.PrimaryURL {
		return errors.New("source discovery and primary URLs are invalid")
	}
	for _, value := range source.EvidenceURLs {
		if validateHTTPS(value) != nil {
			return errors.New("source evidence URL is invalid")
		}
	}
	reviewed, reviewErr := time.Parse(time.DateOnly, reviewedOn)
	revalidate, revalidateErr := time.Parse(time.DateOnly, source.RevalidateBy)
	if reviewErr != nil || revalidateErr != nil || !revalidate.After(reviewed) || revalidate.After(reviewed.AddDate(0, 6, 0)) {
		return errors.New("source revalidation window is invalid")
	}
	return nil
}

func validateHTTPS(value string) error {
	parsed, err := url.Parse(value)
	if err != nil || parsed.Scheme != "https" || parsed.Host == "" || parsed.User != nil || parsed.Fragment != "" {
		return errors.New("URL must use safe HTTPS")
	}
	return nil
}
