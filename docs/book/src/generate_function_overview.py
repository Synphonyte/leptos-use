import os
import sys


def main():
    entry = sys.argv[1]

    module = sys.argv[2] if len(sys.argv) > 2 else None

    generate_function_overview_for_category(entry, None)


def generate_function_overview_for_category(category, module):
    print(f"## {category.title()}")

    listdir = os.listdir(os.path.join(os.getcwd(), category))
    listdir.sort()

    for name in listdir:
        if name.endswith(".md"):
            generate_function_overview(category, name[:-3], module)


def generate_function_overview(category, name, module):
    module = f"/{module}" if module is not None else ""

    file_name = f"../../../src{module}/{name}.rs"
    with open(file_name) as f:
        for line in f.readlines():
            if line.startswith("///"):
                line = line.strip().replace("/// ", "")
                print(f"- [{name}](/{category}/{name}.md) – {line}")
                return


if __name__ == '__main__':
    main()
