import os
import shutil
import subprocess
import sys
import re


def main():
    src_dir = os.path.join(os.getcwd(), "src")
    for dir in os.listdir(src_dir):
        category = dir
        category_dir = os.path.join(src_dir, dir)
        if os.path.isdir(category_dir):
            for file in os.listdir(category_dir):
                if file.endswith(".md") and (len(sys.argv) == 1 or (sys.argv[1] in file)):
                    if build_and_copy_demo(category, file):
                        rewrite_links(category, file)


def build_and_copy_demo(category, md_name):
    name = md_name[:-3]
    example_dir = f"../../examples/{name}"
    if os.path.exists(example_dir):
        p = subprocess.Popen(["trunk", "build", "--release"], cwd=example_dir)
        code = p.wait()

        if code != 0:
            sys.stderr.write(f"failed to build example '{name}'\n")
            sys.exit(code)

        example_output_path = os.path.join(example_dir, "dist")
        target_path = os.path.join("book", category, name, "demo")

        print(f"Copying demo from {example_output_path} -> {target_path}")

        shutil.copytree(example_output_path, target_path,
                        dirs_exist_ok=True)

        with open(os.path.join(target_path, "index.html"), "r") as f:
            html = f.read().replace("/demo", f"./{name}/demo")
            demo_head = html.split("<head>")[1].split("</head>")[0]
            demo_body = html.split("<body>")[1].split("</body>")[0]

        book_html_path = os.path.join("book", category, f"{name}.html")
        with open(book_html_path, "r") as f:
            html = f.read()
            head_split = html.split("<head>")
            target_head = head_split[1].split("</head>")[0]
            body_split = re.split("<body[^>]*>", html)[1].split("</body>")
            target_body = body_split[0]

        with open(book_html_path, "w") as f:
            f.write(
                f"""{head_split[0]} 
<head>
    {demo_head}
    {target_head}
</head>
<body>
    {demo_body}
    {target_body}
</body>
{body_split[1]}""")

        return True

    return False


def rewrite_links(category, md_name):
    """Rewrite links in generated documentation to make them
    compatible between rustdoc and the book.
    """
    html_name = f"{md_name[:-3]}.html"
    target_path = os.path.join("book", category, html_name)

    with open(target_path, "r") as f:
        html = f.read()

    html = html.replace(
        "fn@crate::", "",
    ).replace(
        "crate::", "",
    ).replace(
        "fn@", "",
    )

    with open(target_path, "w") as f:
        f.write(html)


if __name__ == '__main__':
    main()
