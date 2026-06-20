import os
import pathlib


def repo_path(repo, path):
    return pathlib.Path(repo) / path


def is_executable(path):
    return path.exists() and os.access(path, os.X_OK)


def read_if_exists(path):
    return path.read_text(encoding="utf-8") if path.exists() else ""


def has_private_path(text):
    return "/" + "Users" + "/" in text
