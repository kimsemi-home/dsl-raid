package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"

	"github.com/kimsemi-home/dsl-raid/repo-governance/internal/researchradar"
)

func main() {
	if err := run(os.Args[1:]); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func run(args []string) error {
	root, err := repositoryRoot()
	if err != nil {
		return err
	}
	path := filepath.Join(root, "examples", "source-radar", "research-decisions.json")
	if len(args) == 1 {
		path = args[0]
		if !filepath.IsAbs(path) {
			path = filepath.Join(root, filepath.FromSlash(path))
		}
	} else if len(args) != 0 {
		return errors.New("usage: researchradar [manifest-path]")
	}
	manifest, err := researchradar.Read(path)
	if err != nil {
		return err
	}
	report, err := researchradar.BuildReport(manifest)
	if err != nil {
		return err
	}
	body, err := json.MarshalIndent(report, "", "  ")
	if err != nil {
		return err
	}
	fmt.Println(string(body))
	return nil
}

func repositoryRoot() (string, error) {
	root, err := os.Getwd()
	if err != nil {
		return "", err
	}
	for {
		if _, err := os.Stat(filepath.Join(root, "go.mod")); err == nil {
			return root, nil
		}
		parent := filepath.Dir(root)
		if parent == root {
			return "", errors.New("repository root not found")
		}
		root = parent
	}
}
