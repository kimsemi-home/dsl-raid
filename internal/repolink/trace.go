package repolink

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
)

type Manifest struct {
	SchemaVersion string  `json:"schema_version"`
	Groups        []Group `json:"groups"`
}

type Group struct {
	ID                 string   `json:"id"`
	DocumentSources    []string `json:"document_sources"`
	GeneratedDocuments []string `json:"generated_documents"`
	Code               []string `json:"code"`
	Tests              []string `json:"tests"`
	ChangePolicy       string   `json:"change_policy"`
}

func LoadManifest(root string) (Manifest, error) {
	body, err := os.ReadFile(filepath.Join(root, "traceability.json"))
	if err != nil {
		return Manifest{}, err
	}
	var manifest Manifest
	decoder := json.NewDecoder(bytes.NewReader(body))
	decoder.DisallowUnknownFields()
	if err := decoder.Decode(&manifest); err != nil {
		return Manifest{}, err
	}
	var extra any
	if err := decoder.Decode(&extra); !errors.Is(err, io.EOF) {
		return Manifest{}, errors.New("trace manifest must contain one JSON value")
	}
	return manifest, VerifyManifest(root, manifest)
}

func VerifyManifest(root string, manifest Manifest) error {
	if manifest.SchemaVersion != "repo.traceability/v1" || len(manifest.Groups) == 0 {
		return errors.New("trace manifest header is invalid")
	}
	for _, group := range manifest.Groups {
		if group.ID == "" || group.ChangePolicy != "bidirectional" || len(group.DocumentSources) == 0 || len(group.GeneratedDocuments) == 0 || len(group.Code) == 0 || len(group.Tests) == 0 {
			return fmt.Errorf("trace group %q is incomplete", group.ID)
		}
		paths := append(append(append([]string{}, group.DocumentSources...), group.GeneratedDocuments...), append(group.Code, group.Tests...)...)
		for _, rel := range paths {
			if filepath.IsAbs(rel) || filepath.Clean(rel) != rel || strings.HasPrefix(rel, "..") {
				return fmt.Errorf("unsafe trace path %q", rel)
			}
			if _, err := os.Stat(filepath.Join(root, rel)); err != nil {
				return fmt.Errorf("missing trace path %q", rel)
			}
		}
	}
	return nil
}

func StagedFiles(root string) ([]string, error) {
	return gitFiles(root, "diff", "--cached", "--name-only", "--diff-filter=ACMR")
}

func RangeFiles(root, base string) ([]string, error) {
	if base == "" {
		return nil, errors.New("base ref is required")
	}
	return gitFiles(root, "diff", "--name-only", "--diff-filter=ACMR", base+"...HEAD")
}

func gitFiles(root string, args ...string) ([]string, error) {
	cmd := exec.Command("git", args...)
	cmd.Dir = root
	out, err := cmd.Output()
	if err != nil {
		return nil, err
	}
	var files []string
	for _, line := range strings.Split(string(out), "\n") {
		if line = strings.TrimSpace(line); line != "" {
			files = append(files, filepath.ToSlash(line))
		}
	}
	sort.Strings(files)
	return files, nil
}

func CheckChanges(manifest Manifest, files []string) error {
	for _, group := range manifest.Groups {
		source := anyMatch(files, group.DocumentSources)
		generated := anyMatch(files, group.GeneratedDocuments)
		technical := anyMatch(files, append(append([]string{}, group.Code...), group.Tests...))
		if (source || generated || technical) && (source != generated || source != technical) {
			return fmt.Errorf("one-sided document/code change in %s: source=%t generated=%t technical=%t", group.ID, source, generated, technical)
		}
	}
	return nil
}

func anyMatch(files, paths []string) bool {
	for _, file := range files {
		for _, path := range paths {
			if file == path || strings.HasPrefix(file, strings.TrimSuffix(path, "/")+"/") {
				return true
			}
		}
	}
	return false
}
