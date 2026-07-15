package repolink

import "testing"

func TestBidirectionalChanges(t *testing.T) {
	manifest := Manifest{Groups: []Group{{
		ID: "group", DocumentSources: []string{"docs-src"}, GeneratedDocuments: []string{"docs/generated"},
		Code: []string{"internal"}, Tests: []string{"tests"}, ChangePolicy: "bidirectional",
	}}}
	if err := CheckChanges(manifest, []string{"docs-src/a.json", "docs/generated/a.md", "internal/a.go"}); err != nil {
		t.Fatal(err)
	}
	if err := CheckChanges(manifest, []string{"internal/a.go"}); err == nil {
		t.Fatal("expected code-only change to fail")
	}
	if err := CheckChanges(manifest, []string{"docs-src/a.json", "docs/generated/a.md"}); err == nil {
		t.Fatal("expected docs-only change to fail")
	}
}
