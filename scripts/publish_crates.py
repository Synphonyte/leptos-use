#!/usr/bin/env python3
"""Publishes all workspace crates whose current version isn't on crates.io yet.

The crates are published in dependency order, so `leptos-use` comes last. crates.io only allows a
few new crates in a short time and then answers with "429 Too Many Requests" and the time after
which publishing is allowed again. The script waits until then and continues, so it can run over
night:

    caffeinate -i python3 scripts/publish_crates.py

(`caffeinate` keeps the Mac from sleeping.) Crates that are already published are skipped, so the
script can simply be started again after it was interrupted. Use `--dry-run` to only print what
would be published.

Requires `cargo login` with a token that is allowed to publish new crates and a clean working tree
(cargo refuses to publish uncommitted files).
"""

import argparse
import email.utils
import json
import os
import re
import subprocess
import sys
import time
import urllib.error
import urllib.request
from datetime import datetime, timedelta, timezone

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

RATE_LIMIT_RETRY_AFTER = re.compile(r"try again after (\w{3}, \d{1,2} \w{3} \d{4} \d{2}:\d{2}:\d{2} GMT)")
# Wait a bit longer than crates.io asks for.
RATE_LIMIT_MARGIN = timedelta(seconds=30)
# Used when the 429 response doesn't say how long to wait.
RATE_LIMIT_FALLBACK = timedelta(minutes=10)

OTHER_ERROR_ATTEMPTS = 3
OTHER_ERROR_PAUSE = timedelta(minutes=1)


def log(message):
    print(f"[{datetime.now():%Y-%m-%d %H:%M:%S}] {message}", flush=True)


def workspace_packages():
    """Returns {name: (version, [names of the workspace packages it needs on crates.io])}."""
    metadata = json.loads(
        subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            cwd=REPO,
            check=True,
            capture_output=True,
            text=True,
        ).stdout
    )
    names = {p["name"] for p in metadata["packages"]}

    packages = {}
    for package in metadata["packages"]:
        deps = set()
        for dep in package["dependencies"]:
            # dev-dependencies with only a path (requirement `*`) are removed when publishing
            if dep["name"] in names and not (dep["kind"] == "dev" and dep["req"] == "*"):
                deps.add(dep["name"])
        deps.discard(package["name"])
        packages[package["name"]] = (package["version"], sorted(deps))
    return packages


def publish_order(packages):
    """Topological order of the packages, dependencies first (alphabetical where it doesn't matter)."""
    order, done, visiting = [], set(), set()

    def visit(name):
        if name in done:
            return
        if name in visiting:
            raise SystemExit(f"dependency cycle involving {name}")
        visiting.add(name)
        for dep in packages[name][1]:
            visit(dep)
        visiting.remove(name)
        done.add(name)
        order.append(name)

    for name in sorted(packages):
        visit(name)
    return order


def index_path(name):
    name = name.lower()
    if len(name) <= 2:
        return f"{len(name)}/{name}"
    if len(name) == 3:
        return f"3/{name[0]}/{name}"
    return f"{name[:2]}/{name[2:4]}/{name}"


def is_published(name, version):
    url = f"https://index.crates.io/{index_path(name)}"
    try:
        with urllib.request.urlopen(url, timeout=30) as response:
            lines = response.read().decode().splitlines()
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return False
        raise
    return any(json.loads(line)["vers"] == version for line in lines if line.strip())


def retry_after(output):
    """The time crates.io allows publishing again, if `output` is a rate limit error."""
    if "429" not in output and "Too Many Requests" not in output:
        return None
    m = RATE_LIMIT_RETRY_AFTER.search(output)
    if m:
        return email.utils.parsedate_to_datetime(m.group(1)) + RATE_LIMIT_MARGIN
    return datetime.now(timezone.utc) + RATE_LIMIT_FALLBACK


def sleep_until(when):
    seconds = (when - datetime.now(timezone.utc)).total_seconds()
    if seconds > 0:
        log(f"waiting until {when.astimezone():%H:%M:%S} ({seconds / 60:.0f} min)")
        time.sleep(seconds)


def publish(name, extra_args):
    """Publishes `name`, waiting out rate limits. Returns False if it failed for another reason."""
    other_errors = 0
    while True:
        log(f"publishing {name}")
        result = subprocess.run(
            ["cargo", "publish", "-p", name, *extra_args],
            cwd=REPO,
            capture_output=True,
            text=True,
        )
        output = result.stdout + result.stderr
        if result.returncode == 0:
            log(f"published {name}")
            return True

        when = retry_after(output)
        if when is not None:
            log(f"rate limited while publishing {name}")
            sleep_until(when)
            continue

        if "already exists" in output:
            log(f"{name} is already published")
            return True

        other_errors += 1
        log(f"publishing {name} failed (attempt {other_errors}/{OTHER_ERROR_ATTEMPTS}):\n{output.strip()}")
        if other_errors >= OTHER_ERROR_ATTEMPTS:
            return False
        time.sleep(OTHER_ERROR_PAUSE.total_seconds())


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--dry-run", action="store_true", help="only print what would be published")
    parser.add_argument("--allow-dirty", action="store_true", help="passed on to `cargo publish`")
    args = parser.parse_args()

    if not args.allow_dirty and not args.dry_run:
        status = subprocess.run(
            ["git", "status", "--porcelain"], cwd=REPO, check=True, capture_output=True, text=True
        ).stdout
        if status.strip():
            raise SystemExit(f"the working tree isn't clean (commit first or pass --allow-dirty):\n{status}")

    packages = workspace_packages()
    todo = [name for name in publish_order(packages) if not is_published(name, packages[name][0])]

    log(f"{len(todo)} of {len(packages)} crates to publish")
    for name in todo:
        print(f"  {name} {packages[name][0]}")
    if args.dry_run or not todo:
        return

    extra_args = ["--allow-dirty"] if args.allow_dirty else []
    for i, name in enumerate(todo, 1):
        log(f"[{i}/{len(todo)}]")
        if not publish(name, extra_args):
            raise SystemExit(f"stopped: {name} couldn't be published, see the error above")

    log("all crates are published")


if __name__ == "__main__":
    main()
