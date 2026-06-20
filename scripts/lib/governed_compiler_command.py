import subprocess


def run_command(repo, command):
    return subprocess.run(
        ["bash", "-lc", command],
        cwd=repo,
        text=True,
        capture_output=True,
    )


def expected_stdout(row):
    return row.get("assertion", "").removeprefix("stdout:")


def check_stage_command(row, repo, errors):
    result = run_command(repo, row.get("command", ""))
    expected = expected_stdout(row)
    if result.returncode or expected not in result.stdout:
        errors.append(f"{row.get('id', '')} expected stdout {expected!r}")
