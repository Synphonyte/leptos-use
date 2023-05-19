import sys


def main():
    name = sys.argv[1]
    file_name = f"../../../../src/{name}.rs"
    with open(file_name) as f:
        in_code_block = False
        doc_comment_started = False
        for line in f.readlines():
            if line.startswith("///"):
                doc_comment_started = True
                line = line.strip().replace("/// ", "").replace("///", "")
                if "```" in line:
                    if not in_code_block:
                        line = line.replace("```", "```rust,ignore")
                    in_code_block = not in_code_block

                line = process_line(line, name)

                print(line)
            elif doc_comment_started:
                return


def process_line(line, name):
    stripped = line.strip()
    result = line

    if stripped.startswith("[Link to Demo](https://"):
        example_link = stripped.replace("[Link to Demo](", "").replace(")", "")
        result = f'''<div class="demo-container">
    <a class="demo-source" href="{example_link}/src/main.rs" target="_blank">source</a>
    <iframe class="demo" src="{name}/demo/index.html" width="100%" frameborder="0">
    </iframe>
</div>'''
    return result


if __name__ == '__main__':
    main()
