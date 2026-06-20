import pathlib


def repo_path(repo, path):
    return pathlib.Path(repo) / path


def exists(repo, path):
    return repo_path(repo, path).exists()


def has_private_path(text):
    return "/" + "Users" + "/" in text
