import pathlib


def exists(repo, path):
    return (pathlib.Path(repo) / path).exists()


def has_private_path(text):
    local_home = "/" + "Users" + "/"
    linux_home = "/" + "home" + "/"
    return local_home in text or linux_home in text
