import sys
import re
import os


def main():
    category = os.path.split(os.getcwd())[-1]

    name = sys.argv[1]
    file_name = f"../../../../src/{name}.rs"

    module = None
    if "/" in name:
        module, name = name.split("/")

    if module is not None:
        module_display = f"leptos_use::{module}"
    else:
        module_display = "leptos_use"

    feature = sys.argv[2] if len(sys.argv) > 2 else None

    feature_display = ""
    if feature is not None:
        feature_display = f"<div>Feature</div><div>{feature}</div>"

    print(f"""
<div class="meta-data">
    <div>Category</div>
    <div>{category.title()}</div>
    <div>Module</div>
    <div><code>{module_display}</code></div>
    {feature_display}
</div>
    """)

    types = [];

    with open(file_name) as f:
        in_code_block = False
        doc_comment_started = False
        initial_doc_finished = False

        for line in f.readlines():
            stripped_line = line.strip()

            if initial_doc_finished:
                process_further_line(line, types, module)

            elif stripped_line.startswith("///") or stripped_line.startswith("#[doc ="):
                doc_comment_started = True

                if line.startswith("#[doc"):
                    line = stripped_line.replace("#[doc = \" ", "").replace("#[doc = \"", "")[:-2]
                else:
                    line = stripped_line.replace("/// ", "").replace("///", "")

                if "```" in line:
                    if not in_code_block:
                        line = line.replace("```", "```rust,ignore")
                    in_code_block = not in_code_block

                line = process_line(line, name, module)

                print(line)

            elif doc_comment_started:
                initial_doc_finished = True

    if feature is not None:
        append_feature_paragraph(feature)

    add_types_paragraph(types)
    add_source_paragraph(name, module)


def add_types_paragraph(types):
    if types:
        print("\n## Types\n")
        print("\n".join(types))


def add_source_paragraph(name, module):
    print("\n## Source\n")

    if module is not None:
        module = f"/{module}"

    source_url = f"https://github.com/Synphonyte/leptos-use/blob/main/src{module}/{name}.rs"
    demo_url = f"https://github.com/Synphonyte/leptos-use/tree/main/examples/{name}"
    docs_url = f"https://docs.rs/leptos-use/latest/leptos_use{module}/fn.{name}.html"

    demo_link = " • <a href=\"{demo_url}\" target=\"_blank\">Demo</a>" if os.path.isdir(
        os.path.join("..", "..", "..", "..", "examples", name)) else ""

    print(
        f"<a href=\"{source_url}\" target=\"_blank\">Source</a>{demo_link} • <a href=\"{docs_url}\" target=\"_blank\">Docs</a>")


internal_doc_link_pattern = re.compile(r"\[`([^]]+)\`](?!\()")
ident_pattern = re.compile(r"^[a-zA-Z_][a-zA-Z0-9_]*\b")


def process_line(line, name, module):
    stripped = line.strip()
    result = line

    if stripped.startswith("[Link to Demo](https://"):
        example_link = stripped.replace("[Link to Demo](", "").replace(")", "")
        result = f'''<div class="demo-container">
    <a class="demo-source" href="{example_link}/src/main.rs" target="_blank">source <i class="fa fa-github"></i></a>
    <div id="demo-anchor"></div>
</div>'''
    else:
        if module is not None:
            module = f"/{module}"

        result = re.sub(internal_doc_link_pattern,
                        rf"[`\1`](https://docs.rs/leptos-use/latest/leptos_use{module}/fn.\1.html)",
                        line)

    return result


def process_further_line(line, types, module=None):
    if line.startswith("pub enum"):
        append_type(line, "enum", types, module)
    elif line.startswith("pub struct"):
        append_type(line, "struct", types, module)


def append_feature_paragraph(feature):
    print(f"""## Feature
> This function is only available if the crate feature **`{feature}`** is enabled""")


def append_type(line, ty, types, module=None):
    start_index = len(f"pub {ty} ")
    m = re.search(ident_pattern, line[start_index:])
    if m is not None:
        ident = m.group(0)

        if module is not None:
            module = f"/{module}"
        else:
            module = ""

        types.append(f"- [`{ty} {ident}`](https://docs.rs/leptos-use/latest/leptos_use{module}/{ty}.{ident}.html)")


if __name__ == '__main__':
    main()
