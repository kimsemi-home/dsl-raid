package researchradar

type expectedSource struct {
	decision string
	target   string
}

func requiredSources() map[string]expectedSource {
	return map[string]expectedSource{
		"cli-auth-right-way":             {decision: "adapt", target: "myhome-shorts-factory"},
		"devflow-native":                 {decision: "adapt", target: "dsl-raid"},
		"google-device-flow-scope-limit": {decision: "defer", target: "myhome-shorts-factory"},
		"google-installed-app-oauth":     {decision: "adopt", target: "myhome-shorts-factory"},
		"loop-engineering":               {decision: "adapt", target: "dsl-raid"},
		"paisa-local-first-finance":      {decision: "adapt", target: "myhome-ledger"},
	}
}
