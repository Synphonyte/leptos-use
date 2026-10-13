import glob
import os
import re
import sys

REPO = os.path.join("..", "..", "..", "..")


def find_source(name):
    """Returns (crate name, path of the source file relative to the repo root)."""
    base = name.split("/")[-1]
    matches = sorted(glob.glob(os.path.join(REPO, "crates", "*", "src", "**", f"{base}.rs"), recursive=True))
    if not matches:
        raise SystemExit(f"source file for {name} not found")
    rel = os.path.relpath(matches[0], REPO)
    return rel.split(os.sep)[1], rel


_SYMBOLS = None


def crate_of(ident, default):
    """Find the crate that defines `ident` (falls back to `default`)."""
    global _SYMBOLS
    if _SYMBOLS is None:
        _SYMBOLS = {}
        item = re.compile(r"^pub\s+(?:async\s+)?(fn|struct|enum|trait|type)\s+(\w+)", re.M)
        for f in glob.glob(os.path.join(REPO, "crates", "*", "src", "**", "*.rs"), recursive=True):
            crate = os.path.relpath(f, REPO).split(os.sep)[1]
            with open(f) as fh:
                for kind, s in item.findall(fh.read()):
                    _SYMBOLS.setdefault(s, (crate, kind))
    return _SYMBOLS.get(ident, (default, None))


def lib(crate):
    return crate.replace("-", "_")


def main():
    category = os.path.split(os.getcwd())[-1]

    name = sys.argv[1]
    crate, source_path = find_source(name)
    if "/" in name:
        name = name.split("/")[-1]

    print(f"""
<div class="meta-data">
    <div>Category</div>
    <div>{category.title()}</div>
    <div>Crate</div>
    <div><a href="https://crates.io/crates/{crate}" target="_blank"><code>{crate}</code></a></div>
</div>
    """)

    types = []

    with open(os.path.join(REPO, source_path)) as f:
        in_code_block = False
        doc_comment_started = False
        initial_doc_finished = False

        for line in f.readlines():
            stripped_line = line.strip()

            if initial_doc_finished:
                process_further_line(line, types, crate)

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

                line = process_line(line, name, crate)

                print(line)

            elif re.match(r"^//\s?#\[doc\(cfg\(.+?\)\)]", stripped_line):
                pass

            elif doc_comment_started:
                initial_doc_finished = True

    append_installation_paragraph(crate)
    add_types_paragraph(types)
    add_source_paragraph(name, crate, source_path)


def add_types_paragraph(types):
    if types:
        print("\n## Types\n")
        print("\n".join(types))


def add_source_paragraph(name, crate, source_path):
    print("\n## Source\n")

    source_url = f"https://github.com/Synphonyte/leptos-use/blob/main/{source_path}"
    demo_url = f"https://github.com/Synphonyte/leptos-use/tree/main/examples/{name}"
    docs_url = f"https://docs.rs/{crate}/latest/{lib(crate)}/fn.{name}.html"

    demo_link = f" • <a href=\"{demo_url}\" target=\"_blank\">Demo</a>" if os.path.isdir(
        os.path.join(REPO, "examples", name)) else ""

    print(
        f"<a href=\"{source_url}\" target=\"_blank\">Source</a>{demo_link} • <a href=\"{docs_url}\" target=\"_blank\">Docs</a>")


internal_doc_link_pattern = re.compile(r"\[`([^]]+)\`](?!\()")
ident_pattern = re.compile(r"^[a-zA-Z_][a-zA-Z0-9_]*\b")


def process_line(line, name, crate):
    stripped = line.strip()
    result = line

    if stripped.startswith("[Link to Demo](https://"):
        example_link = stripped.replace("[Link to Demo](", "").replace(")", "")
        result = f"""<div class="demo-container">
    <a class="demo-source" href="{example_link}/src/main.rs" target="_blank">source <i class="fa-brands fa-github"></i></a>
    <div id="demo-anchor"></div>
</div>"""
    else:
        def link(m):
            text = m.group(1)
            path = re.sub(r"^(fn|struct|enum|trait|type|macro)@", "", text).replace("crate::", "")
            segs = [x for x in path.split("::") if x not in ("core", "utils", "storage", "math", "docs")]
            target, kind = crate_of(segs[0], crate)
            if kind is None or not re.fullmatch(r"\w+", segs[0]):
                return f"[`{text}`](https://docs.rs/{crate}/latest/{lib(crate)}/fn.{segs[0]}.html)"
            url = f"https://docs.rs/{target}/latest/{lib(target)}/{kind}.{segs[0]}.html"
            if len(segs) > 1:
                url += f"#structfield.{segs[1]}" if kind == "struct" else f"#method.{segs[1]}"
            return f"[`{text}`]({url})"

        result = re.sub(internal_doc_link_pattern, link, line)

    return result


def process_further_line(line, types, crate):
    if line.startswith("pub enum"):
        append_type(line, "enum", types, crate)
    elif line.startswith("pub struct"):
        append_type(line, "struct", types, crate)


def append_installation_paragraph(crate):
    print(f"""## Installation

```shell
cargo add {crate}
```""")


def append_type(line, ty, types, crate):
    start_index = len(f"pub {ty} ")
    m = re.search(ident_pattern, line[start_index:])
    if m is not None:
        ident = m.group(0)
        types.append(f"- [`{ty} {ident}`](https://docs.rs/{crate}/latest/{lib(crate)}/{ty}.{ident}.html)")


if __name__ == '__main__':
    main()
