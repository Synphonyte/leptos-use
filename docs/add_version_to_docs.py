import os
import re
import sys
from datetime import datetime


def main():
    with open("Cargo.toml", "r") as f:
        cargo_text = f.read()

        m = re.search(r'leptos = "([^"]+)"', cargo_text)
        leptos_version = m.group(1)

        m = re.search(r'version = "(\d+)\.(\d+)\.(\d+)"', cargo_text)
        crate_version_short = f"{m.group(1)}.{m.group(2)}"
        crate_version_long = f"{m.group(1)}.{m.group(2)}.{m.group(3)}"

    print("Found crate version", crate_version_short, "and leptos version", leptos_version)

    with open("README.md", "r") as f:
        original_text = f.read()

        text = add_to_compat_table(leptos_version, crate_version_short, original_text)

        if len(sys.argv) > 1 and sys.argv[1] == "--check":
            if original_text != text:
                print("[Failed] README.md doesn't contain the current crate version in the compatibility table",
                      file=sys.stderr)
                print("  * Run `python3 docs/add_version_to_docs.py` to fix", file=sys.stderr)
                quit(1)
            else:
                print("[OK] README.md does contain the current crate version in the compatibility table")

    if len(sys.argv) == 1:
        with open("README.md", "w") as f:
            f.write(text)

    with open("CHANGELOG.md", "r") as f:
        original_text = f.read()

        text = replace_in_changelog(crate_version_long, original_text)

        if len(sys.argv) > 1 and sys.argv[1] == "--check":
            if original_text != text:
                print("[Failed] CHANGELOG.md still contains an [Unreleased] header",
                      file=sys.stderr)
                print("  * Run `python3 docs/add_version_to_docs.py` to fix", file=sys.stderr)
                quit(1)
            else:
                print("[OK] CHANGELOG.md doesn't contain an [Unreleased] header")

    if len(sys.argv) == 1:
        with open("CHANGELOG.md", "w") as f:
            f.write(text)


def add_to_compat_table(leptos_version: str, crate_version: str, original_text: str):
    lines = original_text.splitlines()

    table_row = None

    if re.search(rf"^\|[^|]+\| {leptos_version}", lines[-1]) is not None:
        table_row = lines[-1]

    if table_row is None:
        lines.append(f"| {crate_version} | {leptos_version} |")
    elif re.search(rf"^\| .*? {crate_version}\s*\| {leptos_version}", table_row) is not None:
        return original_text
    else:
        index = table_row.index("|", 1)
        while index > 2 and table_row[index - 1] == " ":
            index -= 1
        lines[-1] = f"{table_row[:index]}, {crate_version}{table_row[index:]}"

    return "\n".join(lines) + '\n'


def replace_in_changelog(crate_version: str, original_text: str):
    today = datetime.today().strftime("%Y-%m-%d")
    return original_text.replace("## [Unreleased] -", f"## [{crate_version}] - {today}")


if __name__ == '__main__':
    main()
