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

        example_output_path = os.path.join(example_dir, "dist")
        target_path = os.path.join("book", category, name, "demo")

        print(f"Copying demo from {example_output_path} to {target_path}")

        shutil.copytree(example_output_path, target_path,
                        dirs_exist_ok=True)

        with open(os.path.join(target_path, "index.html"), "r") as f:
            html = f.read().replace("./demo", f"./{name}/demo")
            head = html.split("<head>")[1].split("</head>")[0]
            body = html.split("<body>")[1].split("</body>")[0]

        book_html_path = os.path.join("book", category, f"{name}.html")
        with open(book_html_path, "r") as f:
            html = f.read()
            head_split = html.split("<head>")
            target_head = head_split[1].split("</head>")[0]
            body_split = html.split("<body>")[1].split("</body>")
            target_body = body_split[0]

        with open(book_html_path, "w") as f:
            f.write(
                f"""{head_split[0]}
<head>
    {head}
    {target_head}
</head>
<body>
    {body}
    {target_body}
</body>
{body_split[1]}""")


if __name__ == '__main__':
    main()
