package main

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"

	"github.com/kimsemi-home/dsl-raid/repo-governance/internal/repolink"
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
	if len(args) == 2 && args[0] == "docs" && args[1] == "generate" {
		return report("documents generated", repolink.GenerateDocuments(root))
	}
	if len(args) == 2 && args[0] == "docs" && args[1] == "check" {
		return report("documents verified", repolink.CheckDocuments(root))
	}
	if len(args) == 2 && args[0] == "trace" && args[1] == "verify" {
		_, err := repolink.LoadManifest(root)
		return report("traceability verified", err)
	}
	if len(args) == 2 && args[0] == "trace" && args[1] == "staged" {
		return traceStaged(root)
	}
	if len(args) == 3 && args[0] == "trace" && args[1] == "range" {
		return traceRange(root, args[2])
	}
	return errors.New("usage: repoctl <docs generate|docs check|trace verify|trace staged|trace range BASE>")
}

func traceStaged(root string) error {
	manifest, err := repolink.LoadManifest(root)
	if err != nil {
		return err
	}
	files, err := repolink.StagedFiles(root)
	if err != nil {
		return err
	}
	return report("staged document/code changes verified", repolink.CheckChanges(manifest, files))
}

func traceRange(root, base string) error {
	manifest, err := repolink.LoadManifest(root)
	if err != nil {
		return err
	}
	files, err := repolink.RangeFiles(root, base)
	if err != nil {
		return err
	}
	return report("range document/code changes verified", repolink.CheckChanges(manifest, files))
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

func report(message string, err error) error {
	if err != nil {
		return err
	}
	fmt.Println(message)
	return nil
}
