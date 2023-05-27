import sys
import re


def main():
    name = sys.argv[1]
    file_name = f"../../../../src/{name}.rs"

    types = [];

    with open(file_name) as f:
        in_code_block = False
        doc_comment_started = False
        initial_doc_finished = False

        for line in f.readlines():
            if initial_doc_finished:
                process_further_line(line, name, types)

            elif line.startswith("///"):
                doc_comment_started = True
                line = line.strip().replace("/// ", "").replace("///", "")
                if "```" in line:
                    if not in_code_block:
                        line = line.replace("```", "```rust,ignore")
                    in_code_block = not in_code_block

                line = process_line(line, name)

                print(line)

            elif doc_comment_started:
                initial_doc_finished = True

    add_types_paragraph(types)
    add_source_paragraph(name)


def add_types_paragraph(types):
    if types:
        print("\n## Types\n")
        print("\n".join(types))


def add_source_paragraph(name):
    print("\n## Source\n")

    source_url = f"https://github.com/Synphonyte/leptos-use/blob/main/src/{name}.rs"
    demo_url = f"https://github.com/Synphonyte/leptos-use/tree/main/examples/{name}"
    docs_url = f"https://docs.rs/leptos-use/latest/leptos_use/fn.{name}.html"

    print(
        f"<a href=\"{source_url}\" target=\"_blank\">Source</a> • <a href=\"{demo_url}\" target=\"_blank\">Demo</a> • <a href=\"{docs_url}\" target=\"_blank\">Docs</a>")


interal_doc_link_pattern = re.compile(r"\[`([^]]+)\`](?!\()")
ident_pattern = re.compile(r"^[a-zA-Z_][a-zA-Z0-9_]*\b")


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


def process_further_line(line, name, types):
    if line.startswith("pub enum"):
        append_type(line, "enum", types)
    elif line.startswith("pub struct"):
        append_type(line, "struct", types)


def append_type(line, ty, types):
    start_index = len(f"pub {ty} ")
    m = re.search(ident_pattern, line[start_index:])
    if m is not None:
        ident = m.group(0)
        types.append(f"- [`{ty} {ident}`](https://docs.rs/leptos-use/latest/leptos_use/{ty}.{ident}.html)")


if __name__ == '__main__':
    main()
