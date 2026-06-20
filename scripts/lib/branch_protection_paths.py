import os
import pathlib


def repo_path(repo, path):
    return pathlib.Path(repo) / path


def exists(repo, path):
    return os.path.exists(repo_path(repo, path))


def read_text(repo, path):
    return repo_path(repo, path).read_text(encoding="utf-8")


def has_private_path(text):
    return "/" + "Users" + "/" in text
