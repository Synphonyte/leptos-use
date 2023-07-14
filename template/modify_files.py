import argparse
import os
import re

parser = argparse.ArgumentParser()


def main():
    parser.add_argument("function_name")
    parser.add_argument("category")
    parser.add_argument("--module", type=str)
    parser.add_argument("--feature", type=str)

    args = parser.parse_args()

    modify_changelog(args)
    modify_librs(args)
    modify_modrs(args)
    modify_summarymd(args)
    modify_workspace(args)


def modify_workspace(args):
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

    with open(config_path, "w") as f:
        f.write(before_members + "members = [\n" + "\n".join(members) + "\n]" + after_members)


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
        println(
            f"Category {args.category} not found in docs/book/src/SUMMARY.md. Please add it manually together with the link to the new function")


def modify_modrs(args):
    if args.module is not None:
        mod_path = os.path.join("src", args.module, "mod.rs")
        if os.path.exists(mod_path):
            with open(mod_path, "r") as f:
                mod_source = f.read()

            mod_source = mod_source.replace("mod", f"mod {args.function_name};\nmod", 1)
            mod_source = mod_source.replace("pub use", f"pub use {args.function_name}::*;\npub use", 1)
        else:
            mod_source = f"#![doc(cfg(feature = \"{args.feature}\"))]\n" if args.feature is not None else ""
            mod_source += f"""
mod {args.function_name};

pub use {args.function_name}::*;
"""

        with open(mod_path, "w") as f:
            f.write(mod_source)


def modify_librs(args):
    with open("src/lib.rs", "r") as f:
        lib_source = f.read()

    feature_prefix = "" if args.feature is None else f"#[cfg(feature = \"{args.feature}\"))]\n"

    if args.module is None:
        lib_source = lib_source.replace("mod on_click_outside;",
                                        f"mod on_click_outside;\n{feature_prefix}mod {args.function_name};")
        lib_source = lib_source.replace("pub use on_click_outside::*;",
                                        f"pub use on_click_outside::*;\n{feature_prefix}pub use {args.function_name}::*;")
    elif args.module not in lib_source:
        lib_source = lib_source.replace("pub mod utils;", f"pub mod utils;\n{feature_prefix}pub mod {args.module};")

    with open("src/lib.rs", "w") as f:
        f.write(lib_source)


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

    changelog_source.insert(9, f"- `{args.function_name}`\n")

    with open("CHANGELOG.md", "w") as f:
        f.write("".join(changelog_source))


if __name__ == '__main__':
    main()
