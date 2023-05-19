import os
import shutil
import subprocess


def main():
    src_dir = os.path.join(os.getcwd(), "src")
    for dir in os.listdir(src_dir):
        category = dir
        category_dir = os.path.join(src_dir, dir)
        if os.path.isdir(category_dir):
            for file in os.listdir(category_dir):
                if file.endswith(".md"):
                    build_and_copy_demo(category, file)


def build_and_copy_demo(category, md_name):
    name = md_name[:-3]
    example_dir = f"../../examples/{name}"
    if os.path.exists(example_dir):
        p = subprocess.Popen(["trunk", "build"], cwd=example_dir)
        p.wait()
        shutil.copytree(os.path.join(example_dir, "dist"), os.path.join("book", category, name, "demo"),
                        dirs_exist_ok=True)


if __name__ == '__main__':
    main()
