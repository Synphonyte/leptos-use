import glob
import os
import sys


def main():
    entry = sys.argv[1]

    module = sys.argv[2] if len(sys.argv) > 2 else None

    generate_function_overview_for_category(entry, module)


def generate_function_overview_for_category(category, module):
    print(f"## {category.title()}")

    listdir = os.listdir(os.path.join(os.getcwd(), category))
    listdir.sort()

    for name in listdir:
        if name.endswith(".md"):
            generate_function_overview(category, name[:-3], module)


def generate_function_overview(category, name, module):
    matches = sorted(glob.glob(f"../../../crates/*/src/**/{name}.rs", recursive=True))
    if not matches:
        raise SystemExit(f"source file for {name} not found")
    with open(matches[0]) as f:
        for line in f.readlines():
            stripped_line = line.strip()
            if stripped_line.startswith("///"):
                line = stripped_line.replace("/// ", "")
                print(f"- [{name}](/{category}/{name}.md) – {line}")
                return


if __name__ == '__main__':
    main()
