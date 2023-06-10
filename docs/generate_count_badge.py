import os
import re
import sys

def main():
    count = 0
    for dir in os.listdir("docs/book/src/"):
        dir_path = os.path.join("docs/book/src/", dir)
        if os.path.isdir(dir_path):
            for file in os.listdir(dir_path):
                if file.endswith(".md"):
                    count += 1

    print(f"Found {count} functions")

    with open("README.md", "r") as f:
        original_text = f.read()
        text = re.sub(
            r'<img src="https://img.shields.io/badge/-\d+%20functions-%23EF3939" alt="\d+ Functions"',
            f'<img src="https://img.shields.io/badge/-{count}%20functions-%23EF3939" alt="{count} Functions"',
            original_text
        )

        if sys.argv[1] == "--check":
            if original_text != text:
                print("[Failed] README.md doesn't have the correct function count badge", file=sys.stderr)
                print("  * Run `python3 docs/generate_count_badge.py` to fix", file=sys.stderr)
                quit(1)
            else:
                print("[OK] README.md has the correct function count badge")
                quit(0)

    with open("README.md", "w") as f:
        f.write(text)

if __name__ == '__main__':
    main()
