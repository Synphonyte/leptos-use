#!/usr/bin/env python3
"""Minimal mdbook preprocessor replacing `<!-- cmdrun <command> -->` with the command's stdout."""
import json
import os
import re
import subprocess
import sys

CMDRUN = re.compile(r"<!--\s*cmdrun\s+(.*?)\s*-->")


def run(cmd, cwd):
    p = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True)
    if p.returncode != 0:
        sys.stderr.write(f"cmdrun: `{cmd}` in {cwd} failed:\n{p.stderr}")
        sys.exit(1)
    return p.stdout


def process(items, src_dir):
    for item in items:
        chapter = item.get("Chapter") if isinstance(item, dict) else None
        if not chapter:
            continue
        cwd = src_dir
        if chapter.get("source_path"):
            cwd = os.path.join(src_dir, os.path.dirname(chapter["source_path"]))
        chapter["content"] = CMDRUN.sub(lambda m: run(m.group(1), cwd), chapter["content"])
        process(chapter.get("sub_items", []), src_dir)


def main():
    if len(sys.argv) > 1 and sys.argv[1] == "supports":
        sys.exit(0)
    ctx, book = json.load(sys.stdin)
    src_dir = os.path.join(ctx["root"], ctx["config"]["book"].get("src", "src"))
    process(book.get("items", book.get("sections", [])), src_dir)
    json.dump(book, sys.stdout)


if __name__ == "__main__":
    main()
