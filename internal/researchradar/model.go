package researchradar

const (
	ManifestSchema = "dslraid.research-radar/v1"
	ReportSchema   = "dslraid.research-radar-verification/v1"
)

type Manifest struct {
	SchemaVersion string   `json:"schema_version"`
	ReviewedOn    string   `json:"reviewed_on"`
	Sources       []Source `json:"sources"`
	Checks        []string `json:"checks"`
	AggregateHash string   `json:"aggregate_hash"`
}

type Source struct {
	ID           string   `json:"id"`
	DiscoveryURL string   `json:"discovery_url"`
	PrimaryURL   string   `json:"primary_url"`
	EvidenceURLs []string `json:"evidence_urls"`
	Topic        string   `json:"topic"`
	Target       string   `json:"target"`
	Decision     string   `json:"decision"`
	Action       string   `json:"action"`
	Rationale    string   `json:"rationale"`
	Guardrails   []string `json:"guardrails"`
	RevalidateBy string   `json:"revalidate_by"`
}

type Report struct {
	SchemaVersion   string   `json:"schema_version"`
	SourceCount     int      `json:"source_count"`
	AdoptCount      int      `json:"adopt_count"`
	AdaptCount      int      `json:"adapt_count"`
	DeferCount      int      `json:"defer_count"`
	Targets         []string `json:"targets"`
	ExternalNetwork bool     `json:"external_network"`
	ExternalWrites  bool     `json:"external_writes"`
	AggregateHash   string   `json:"aggregate_hash"`
	Checks          []string `json:"checks"`
	ReportHash      string   `json:"report_hash"`
}
