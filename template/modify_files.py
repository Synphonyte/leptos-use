import argparse
import os
import re
import shutil

parser = argparse.ArgumentParser()


def main():
    parser.add_argument("function_name")
    parser.add_argument("category")
    parser.add_argument("crate_name")

    args = parser.parse_args()

    modify_workspace_root(args)
    copy_licenses(args)
    modify_changelog(args)
    modify_summarymd(args)
    modify_examples_workspace(args)


def insert_sorted_line(lines, start, end_marker, new_line):
    """Insert `new_line` into the sorted block of lines starting at `start` up to `end_marker`."""
    end = start
    while end < len(lines) and not lines[end].startswith(end_marker):
        end += 1
    block = lines[start:end] + [new_line]
    block.sort()
    return lines[:start] + block + lines[end:]


def modify_workspace_root(args):
    with open("Cargo.toml", "r") as f:
        lines = f.read().split("\n")

    i = lines.index("members = [") + 1
    lines = insert_sorted_line(lines, i, "]", f'    "crates/{args.crate_name}",')

    i = lines.index("[workspace.dependencies]") + 1
    j = i
    while lines[j].startswith("leptos-use-"):
        j += 1
    version = re.search(r'version = "([^"]+)"', "\n".join(lines)).group(1)
    block = lines[i:j] + [f'{args.crate_name} = {{ path = "crates/{args.crate_name}", version = "{version}" }}']
    block.sort()
    lines = lines[:i] + block + lines[j:]

    with open("Cargo.toml", "w") as f:
        f.write("\n".join(lines))


def copy_licenses(args):
    for lic in ("LICENSE-MIT", "LICENSE-APACHE"):
        shutil.copy(lic, os.path.join("crates", args.crate_name, lic))


def modify_examples_workspace(args):
    config_path = os.path.join("examples", "Cargo.toml")

    with open(config_path, "r") as f:
        config_source = f.read()

    before_members, after_members = config_source.split("members = [\n", 1)
    parts = after_members.split("]", 1)
    members = parts[0]
    after_members = parts[1] if len(parts) > 1 else ""

    members = members.splitlines()
    members.append(f"    \"{args.function_name}\",")
    members.sort()

    config_source = before_members + "members = [\n" + "\n".join(members) + "\n]" + after_members
    config_source = config_source.replace(
        "[workspace.dependencies]\n",
        f'[workspace.dependencies]\n{args.crate_name} = {{ path = "../crates/{args.crate_name}" }}\n',
        1,
    )

    with open(config_path, "w") as f:
        f.write(config_source)


def modify_summarymd(args):
    summary_path = os.path.join("docs", "book", "src", "SUMMARY.md")

    with open(summary_path, "r") as f:
        summary_source = f.read()

    function_link = f"- [{args.function_name}]({args.category}/{args.function_name}.md)"

    m = re.search(rf"# @?{args.category}\n\n", summary_source, re.IGNORECASE)
    if m:

        parts = summary_source[m.end():].split("\n# ", 1)

        functions = parts[0]
        after = f"\n# {parts[1]}" if len(parts) > 1 else ""

        functions = list(filter(lambda x: x.strip() != "", functions.splitlines()))
        functions.append(function_link)
        functions.sort()

        summary_source = summary_source[:m.end()] + "\n".join(functions) + "\n" + after

        with open(summary_path, "w") as f:
            f.write(summary_source)
    else:
        print(
            f"Category {args.category} not found in docs/book/src/SUMMARY.md. Please add it manually together with the link to the new function")


def modify_changelog(args):
    with open("CHANGELOG.md", "r") as f:
        changelog_source = f.readlines()

    unreleased_heading_exists = False
    for line in changelog_source:
        if line.startswith("## [Unreleased"):
            unreleased_heading_exists = True
            break

    if not unreleased_heading_exists:
        changelog_source.insert(5, "## [Unreleased] - \n")
        changelog_source.insert(6, "\n")

    new_function_heading_exists = False
    for line in changelog_source:
        if line.startswith("### New Functions"):
            new_function_heading_exists = True
            break
        elif line.startswith("## [") and not line.startswith("## [Unreleased"):
            break

    if not new_function_heading_exists:
        changelog_source.insert(7, "### New Functions 🚀\n")
        changelog_source.insert(8, "\n")
        changelog_source.insert(9, "\n")

    changelog_source.insert(9, f"- `{args.function_name}` (crate `{args.crate_name}`)\n")

    with open("CHANGELOG.md", "w") as f:
        f.write("".join(changelog_source))


if __name__ == '__main__':
    main()
