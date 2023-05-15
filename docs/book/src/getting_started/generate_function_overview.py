import os
import sys


def main():
    for entry in sys.argv[1:]:
        generate_function_overview_for_category(entry)


def generate_function_overview_for_category(category):
    print(f"## {category.title()}")

    listdir = os.listdir(os.path.join(os.getcwd(), "..", category))
    listdir.sort()

    for name in listdir:
        if name.endswith(".md"):
            generate_function_overview(category, name[:-3])


def generate_function_overview(category, name):
    file_name = f"../../../../src/{name}.rs"
    with open(file_name) as f:
        in_code_block = False
        for line in f.readlines():
            if line.startswith("///"):
                line = line.strip().replace("/// ", "")
                print(f"- [{name}](/{category}/{name}.md) – {line}")
                return


if __name__ == '__main__':
    main()
