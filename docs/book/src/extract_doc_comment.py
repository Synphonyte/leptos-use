import sys
import re


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


interal_doc_link_pattern = re.compile(r"\[`([^]]+)\`](?!\()")


def process_line(line, name):
    stripped = line.strip()
    result = line

    if stripped.startswith("[Link to Demo](https://"):
        example_link = stripped.replace("[Link to Demo](", "").replace(")", "")
        result = f'''<div class="demo-container">
    <a class="demo-source" href="{example_link}/src/main.rs" target="_blank">source <i class="fa fa-github"></i></a>
    <div id="demo-anchor"></div>
</div>'''

    else:
        result = re.sub(interal_doc_link_pattern,
                        r"[`\1`](https://docs.rs/leptos-use/latest/leptos_use/fn.\1.html)",
                        line)

    return result


if __name__ == '__main__':
    main()
