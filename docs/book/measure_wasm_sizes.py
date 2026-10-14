#!/usr/bin/env python3
"""Estimates how much each function adds to the gzipped wasm binary of an app.

For every book page two release builds of a small `hydrate` Leptos app are compared:

- the baseline app, which depends on all third-party crates the function's crate pulls in
  (directly or through other leptos-use crates), because an app implementing the functionality
  itself would need those too, and
- the same app with the `Demo` component of the function's first doctest instead of an empty one.

leptos-use crates and `default-struct-builder` are not part of the baseline. Both binaries are
processed with `wasm-bindgen` and `wasm-opt -Oz` and gzipped. The baseline app uses some common
Leptos features (`Show`, `For`, `Effect`, `Memo`, `NodeRef`) and formats integers and floats, so
these aren't counted towards the functions. The difference is written to
`wasm_sizes.json` which `src/extract_doc_comment.py` shows on the book pages.

Usage: python3 measure_wasm_sizes.py [--force] [--work-dir DIR] [function ...]

Functions whose crate, workspace crate dependencies, demo and the workspace dependency versions are
unchanged since the last measurement in `wasm_sizes.json` are skipped unless `--force` is given.

Requires the `wasm32-unknown-unknown` target, a `wasm-bindgen` CLI matching the locked
`wasm-bindgen` version and `wasm-opt` (binaryen). Both are looked up on `PATH`, in the caches of
Trunk and cargo-leptos or taken from the `WASM_BINDGEN` and `WASM_OPT` environment variables.
"""
import argparse
import glob
import gzip
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tomllib

BOOK = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(BOOK))
SRC = os.path.join(BOOK, "src")
OUTPUT = os.path.join(BOOK, "wasm_sizes.json")

# Crates the user wouldn't use when implementing a function without leptos-use.
INTERNAL_EXTERNALS = {"default-struct-builder", "sendwrap-fn", "wasm-js-macros"}

RUSTFLAGS = "--cfg=web_sys_unstable_apis"

WASM_OPT_FEATURES = [
    "--enable-bulk-memory",
    "--enable-mutable-globals",
    "--enable-nontrapping-float-to-int",
    "--enable-sign-ext",
    "--enable-reference-types",
    "--enable-multivalue",
]

APP = """\
#![allow(warnings)]

use leptos::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
{demo_mod}
#[component]
fn App() -> impl IntoView {{
    let (count, set_count) = signal(0);
    let double = Memo::new(move |_| count.get() * 2);
    let ratio = Memo::new(move |_| count.get() as f64 / 3.0);
    let el = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {{
        leptos::logging::log!("{{}}", count.get());
    }});

    view! {{
        <div node_ref=el class="app">
            <button on:click=move |_| set_count.update(|c| *c += 1)>"Increment"</button>
            <p>"Count: " {{count}} " Double: " {{double}} " Ratio: " {{ratio}}</p>
            <p>{{move || format!("{{:.2}} of {{}}", ratio.get(), count.get())}}</p>
            <input type="text" prop:value=move || count.get().to_string() />
            <Show when=move || count.get() % 2 == 0>
                <span>"even"</span>
            </Show>
            <ul>
                <For each=move || 0..count.get() key=|i| *i let:i>
                    <li>{{i}}</li>
                </For>
            </ul>
            {demo}
        </div>
    }}
}}

#[wasm_bindgen]
pub fn hydrate() {{
    leptos::mount::hydrate_body(App);
}}
"""

# The baseline mounts an empty component, so only the code inside the doctest's `Demo` is counted.
EMPTY_DEMO = """\
use leptos::prelude::*;

#[component]
fn Demo() -> impl IntoView {
    view! {}
}
"""

# Demos for functions whose first doctest uses a lot of code that doesn't belong to the function.
STORAGE_DEMO = """\
use leptos::prelude::*;
use leptos_use_storage::{{{fn}}};
use codee::string::FromToStringCodec;

#[component]
fn Demo() -> impl IntoView {{
    let (count, set_count, remove_count) = {call}::<i32, FromToStringCodec>({args}"count");

    view! {{
        <button on:click=move |_| set_count.update(|c| *c += 1)>{{count}}</button>
        <button on:click=move |_| remove_count()>"Reset"</button>
    }}
}}
"""

DEMO_OVERRIDES = {
    # the doctest shows three codecs (JSON, Prost, Base64)
    "use_local_storage": STORAGE_DEMO.format(fn="use_local_storage", call="use_local_storage", args=""),
    "use_session_storage": STORAGE_DEMO.format(fn="use_session_storage", call="use_session_storage", args=""),
    "use_storage": STORAGE_DEMO.format(
        fn="use_storage, StorageType", call="use_storage", args="StorageType::Local, "
    ),
}

MANIFEST = """\
[package]
name = "wasm-size-app"
version = "0.0.0"
edition = "2024"
publish = false

[lib]
crate-type = ["cdylib"]

[dependencies]
{dependencies}

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"

[workspace]
"""


def toml_value(value):
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, str):
        return json.dumps(value)
    if isinstance(value, list):
        return "[" + ", ".join(toml_value(v) for v in value) + "]"
    if isinstance(value, dict):
        return "{ " + ", ".join(f"{k} = {toml_value(v)}" for k, v in value.items()) + " }"
    raise TypeError(value)


def load_toml(path):
    with open(path, "rb") as f:
        return tomllib.load(f)


WORKSPACE_DEPS = load_toml(os.path.join(REPO, "Cargo.toml"))["workspace"]["dependencies"]


def resolve_spec(name, spec, base_dir=REPO):
    """Turns a `{ workspace = true, ... }` dependency spec into a standalone one."""
    spec = {"version": spec} if isinstance(spec, str) else dict(spec)
    if "path" in spec:
        spec["path"] = os.path.normpath(os.path.join(base_dir, spec["path"]))
    if spec.pop("workspace", False):
        base = WORKSPACE_DEPS[name]
        base = {"version": base} if isinstance(base, str) else dict(base)
        features = base.get("features", []) + spec.pop("features", [])
        base.update(spec)
        if features:
            base["features"] = sorted(set(features))
        spec = base
    spec.pop("optional", None)
    if "path" in spec and not os.path.isabs(spec["path"]):
        spec["path"] = os.path.join(REPO, spec["path"])
    return spec


def crate_dir(crate):
    return os.path.join(REPO, "crates", crate)


def crate_dev_dependencies(crate):
    manifest = load_toml(os.path.join(crate_dir(crate), "Cargo.toml"))
    deps = dict(manifest.get("dev-dependencies", {}))
    for target in manifest.get("target", {}).values():
        deps.update(target.get("dev-dependencies", {}))
    return deps


def pages():
    """Returns {function name: source path} for all function pages of the book."""
    result = {}
    pattern = re.compile(r"<!--\s*cmdrun python3 \.\./extract_doc_comment\.py (\S+)")
    for md in sorted(glob.glob(os.path.join(SRC, "*", "*.md"))):
        with open(md) as f:
            m = pattern.search(f.read())
        if m:
            name = m.group(1).split("/")[-1]
            matches = sorted(glob.glob(os.path.join(REPO, "crates", "*", "src", "**", f"{name}.rs"), recursive=True))
            if matches:
                result[name] = matches[0]
    return result


def crate_of(path):
    return os.path.relpath(path, REPO).split(os.sep)[1]


def doc_code_blocks(path):
    """Yields the rust code blocks of all doc comments in a file (hidden lines included)."""
    with open(path) as f:
        lines = f.read().splitlines()

    lang, code = None, None
    for line in lines:
        stripped = line.strip()
        if not stripped.startswith("///"):
            continue
        text = stripped[4:] if stripped.startswith("/// ") else stripped[3:]
        if text.lstrip().startswith("```"):
            if code is None:
                lang, code = text.lstrip()[3:].strip(), []
            else:
                if all(t in ("", "rust", "no_run") for t in lang.split(",")):
                    yield "\n".join(code)
                lang, code = None, None
        elif code is not None:
            code.append(text)


def unhide(code):
    lines = []
    for line in code.splitlines():
        stripped = line.lstrip()
        if stripped == "#":
            continue
        if stripped.startswith("# "):
            line = line[: len(line) - len(stripped)] + stripped[2:]
        elif stripped.startswith("##"):
            line = line[: len(line) - len(stripped)] + stripped[1:]
        lines.append(line)
    return "\n".join(lines)


def find_demo(name, source_path):
    """The code of the first doctest of `name` that defines a `Demo` component.

    Doctests that don't use crates which are only dev-dependencies (like `rand`) are preferred, as
    their code would be counted towards the function.
    """
    if name in DEMO_OVERRIDES:
        return DEMO_OVERRIDES[name]

    crate = crate_of(source_path)
    manifest = load_toml(os.path.join(crate_dir(crate), "Cargo.toml"))
    dev_only = {d.replace("-", "_") for d in crate_dev_dependencies(crate)} - {
        d.replace("-", "_") for d in manifest.get("dependencies", {})
    }

    calls = re.compile(rf"\b{name}(?:_with_options|_local)?\s*(?:::<|\(|!)")
    paths = [source_path] + sorted(glob.glob(os.path.join(crate_dir(crate), "src", "**", "*.rs"), recursive=True))
    demos = [
        code
        for path in paths
        for code in map(unhide, doc_code_blocks(path))
        if "fn Demo" in code and calls.search(code)
    ]
    for code in demos:
        if not dev_only & set(re.findall(r"\b([a-z][a-z0-9_]*)::", code)):
            return code
    return demos[0] if demos else None


def run(cmd, cwd, env=None):
    p = subprocess.run(cmd, cwd=cwd, env=env, capture_output=True, text=True)
    if p.returncode != 0:
        raise RuntimeError(f"`{' '.join(cmd)}` failed:\n{p.stderr[-6000:]}")
    return p.stdout


def find_tool(env_var, binary, version, cache_globs):
    if os.environ.get(env_var):
        return os.environ[env_var]
    candidates = [shutil.which(binary)] + sorted(
        p for g in cache_globs for p in glob.glob(os.path.expanduser(g))
    )
    for c in candidates:
        if c and os.access(c, os.X_OK):
            if version is None or version in run([c, "--version"], cwd=REPO):
                return c
    raise SystemExit(
        f"`{binary}` {version or ''} not found. Install it or set {env_var}."
        + (f" (cargo install wasm-bindgen-cli --version {version})" if binary == "wasm-bindgen" else "")
    )


class App:
    def __init__(self, work_dir):
        self.dir = work_dir
        os.makedirs(os.path.join(self.dir, "src"), exist_ok=True)
        self.leptos = resolve_spec("leptos", {"workspace": True})
        self.leptos["features"] = ["hydrate"]
        self.wasm_bindgen = resolve_spec("wasm-bindgen", {"workspace": True})
        self.wasm_bindgen_cli = None
        self.wasm_opt = None

    def write(self, dependencies, demo):
        deps = {"leptos": self.leptos, "wasm-bindgen": self.wasm_bindgen, **dependencies}
        with open(os.path.join(self.dir, "Cargo.toml"), "w") as f:
            f.write(MANIFEST.format(dependencies="\n".join(f"{k} = {toml_value(v)}" for k, v in deps.items())))

        if demo is None:
            app = APP.format(demo_mod="", demo="")
        else:
            app = APP.format(demo_mod="\nmod demo;\nuse demo::Demo;\n", demo="<Demo />")
            demo = re.sub(r"^(\s*)fn Demo\b", r"\1pub fn Demo", demo, flags=re.M)
            with open(os.path.join(self.dir, "src", "demo.rs"), "w") as f:
                f.write("#![allow(warnings)]\n\n" + demo + "\n")
        with open(os.path.join(self.dir, "src", "lib.rs"), "w") as f:
            f.write(app)

        lock = os.path.join(REPO, "Cargo.lock")
        if os.path.exists(lock):
            shutil.copy(lock, os.path.join(self.dir, "Cargo.lock"))

    def metadata(self):
        return json.loads(
            run(
                ["cargo", "metadata", "--format-version", "1", "--filter-platform", "wasm32-unknown-unknown"],
                cwd=self.dir,
            )
        )

    def build(self):
        """Builds the app and returns the size of the gzipped, optimized wasm file."""
        env = dict(os.environ, RUSTFLAGS=RUSTFLAGS)
        env.pop("CARGO_TARGET_DIR", None)
        run(["cargo", "build", "--release", "--lib", "--target", "wasm32-unknown-unknown"], cwd=self.dir, env=env)

        if self.wasm_bindgen_cli is None:
            version = next(
                p["version"] for p in self.metadata()["packages"] if p["name"] == "wasm-bindgen"
            )
            self.wasm_bindgen_cli = find_tool(
                "WASM_BINDGEN",
                "wasm-bindgen",
                version,
                [
                    f"~/Library/Caches/dev.trunkrs.trunk/wasm-bindgen-{version}/wasm-bindgen",
                    f"~/.cache/trunk/wasm-bindgen-{version}/wasm-bindgen",
                ],
            )
            self.wasm_opt = find_tool(
                "WASM_OPT",
                "wasm-opt",
                None,
                [
                    "~/Library/Caches/dev.trunkrs.trunk/wasm-opt-*/bin/wasm-opt",
                    "~/.cache/trunk/wasm-opt-*/bin/wasm-opt",
                    "~/Library/Caches/cargo-leptos/wasm-opt-*/wasm-opt-*/bin/wasm-opt",
                ],
            )

        out = os.path.join(self.dir, "out")
        shutil.rmtree(out, ignore_errors=True)
        wasm = os.path.join(self.dir, "target", "wasm32-unknown-unknown", "release", "wasm_size_app.wasm")
        run([self.wasm_bindgen_cli, "--target", "web", "--out-dir", out, wasm], cwd=self.dir)
        bg = os.path.join(out, "wasm_size_app_bg.wasm")
        run([self.wasm_opt, "-Oz", *WASM_OPT_FEATURES, bg, "-o", bg], cwd=self.dir)
        with open(bg, "rb") as f:
            return len(gzip.compress(f.read(), compresslevel=9))

    def target_size(self):
        total = 0
        for root, _, files in os.walk(os.path.join(self.dir, "target")):
            for name in files:
                try:
                    total += os.path.getsize(os.path.join(root, name))
                except OSError:
                    pass
        return total


def is_internal(package):
    return package["source"] is None or package["name"] in INTERNAL_EXTERNALS


def is_proc_macro(package):
    return any("proc-macro" in t["kind"] for t in package["targets"])


def third_party_dependencies(metadata, crate):
    """The third-party crates `crate` pulls in directly or through other leptos-use crates."""
    packages = {p["id"]: p for p in metadata["packages"]}
    nodes = {n["id"]: n for n in metadata["resolve"]["nodes"]}

    start = next(p["id"] for p in metadata["packages"] if p["name"] == crate and p["source"] is None)
    seen, stack, external = {start}, [start], set()
    while stack:
        node = nodes[stack.pop()]
        for dep in node["deps"]:
            if not any(k["kind"] is None for k in dep["dep_kinds"]) or is_proc_macro(packages[dep["pkg"]]):
                continue
            if is_internal(packages[dep["pkg"]]):
                if dep["pkg"] not in seen:
                    seen.add(dep["pkg"])
                    stack.append(dep["pkg"])
            else:
                external.add(dep["pkg"])

    names = [packages[p]["name"] for p in external]
    deps = {}
    for pkg_id in sorted(external):
        package = packages[pkg_id]
        if package["name"] in ("leptos", "wasm-bindgen"):
            continue
        # several versions of the same crate need different dependency names
        alias = package["name"]
        if names.count(alias) > 1:
            alias = re.sub(r"\W", "_", f"{alias}_{package['version']}")
        features = [f for f in nodes[pkg_id]["features"] if f != "default"]
        deps[alias] = {
            "package": package["name"],
            "version": f"={package['version']}",
            "default-features": "default" in nodes[pkg_id]["features"],
            "features": sorted(features),
        }
    return deps


def demo_dependencies(crate, demo):
    """Dependencies of the variant app: the crate itself plus crates the doctest references."""
    deps = {crate: {"path": crate_dir(crate)}}
    # Unused dependencies don't change the binary, so all third-party dev-dependencies are added
    # (doctests may need ones they don't reference, e.g. `getrandom` with the `wasm_js` feature).
    for name, spec in crate_dev_dependencies(crate).items():
        if name not in ("leptos", "wasm-bindgen", "wasm-bindgen-test") and not name.startswith("leptos-use"):
            deps[name] = resolve_spec(name, spec, crate_dir(crate))
    used = set(re.findall(r"\b([a-z][a-z0-9_]*)::", demo))
    for lib in used:
        if lib.startswith("leptos_use_") and os.path.isdir(crate_dir(lib.replace("_", "-"))):
            deps.setdefault(lib.replace("_", "-"), {"path": crate_dir(lib.replace("_", "-"))})
    return deps


def internal_crates(crate):
    """`crate` and all workspace crates it depends on (for its doctests, too)."""
    seen, stack = {crate}, [crate]
    while stack:
        current = stack.pop()
        manifest = load_toml(os.path.join(crate_dir(current), "Cargo.toml"))
        sections = [manifest.get("dependencies", {})]
        sections += [t.get("dependencies", {}) for t in manifest.get("target", {}).values()]
        if current == crate:
            sections.append(crate_dev_dependencies(crate))
        for deps in sections:
            for name, spec in deps.items():
                spec = resolve_spec(name, spec, crate_dir(current))
                if "path" in spec:
                    dep = os.path.basename(os.path.normpath(spec["path"]))
                    if dep not in seen and os.path.isdir(crate_dir(dep)):
                        seen.add(dep)
                        stack.append(dep)
    return sorted(seen)


def fingerprint(name, source_path):
    """Hash of everything a measurement depends on except the toolchain and third-party crates.

    These are the measured crate and the workspace crates it depends on (manifests and sources),
    the workspace's dependency versions, the demo and this script.
    """
    h = hashlib.sha256()
    h.update(json.dumps(WORKSPACE_DEPS, sort_keys=True).encode())
    h.update((find_demo(name, source_path) or "").encode())
    with open(__file__, "rb") as f:
        h.update(f.read())

    for crate in internal_crates(crate_of(source_path)):
        files = [os.path.join(crate_dir(crate), "Cargo.toml"), os.path.join(crate_dir(crate), "build.rs")]
        files += sorted(glob.glob(os.path.join(crate_dir(crate), "src", "**", "*"), recursive=True))
        for path in files:
            if os.path.isfile(path):
                h.update(os.path.relpath(path, REPO).encode())
                with open(path, "rb") as f:
                    h.update(f.read())
    return h.hexdigest()


def measure(app, name, source_path, cache):
    crate = crate_of(source_path)
    demo = find_demo(name, source_path)
    if demo is None:
        return None

    variant_deps = demo_dependencies(crate, demo)
    app.write(variant_deps, None)
    baseline_deps = third_party_dependencies(app.metadata(), crate)
    # third-party dev-dependencies are part of the baseline
    baseline_deps.update({k: v for k, v in variant_deps.items() if "path" not in v})

    key = json.dumps(baseline_deps, sort_keys=True)
    if key not in cache:
        app.write(baseline_deps, EMPTY_DEMO)
        cache[key] = app.build()
    baseline = cache[key]

    app.write({**baseline_deps, **variant_deps}, demo)
    with_crate = app.build()

    return {"crate": crate, "baseline": baseline, "with_crate": with_crate, "diff": with_crate - baseline}


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("functions", nargs="*", help="functions to measure (default: all)")
    parser.add_argument("--work-dir", default=os.path.join(REPO, "target", "wasm-size-app"))
    parser.add_argument(
        "--force", action="store_true", help="measure functions even if nothing they depend on changed"
    )
    parser.add_argument(
        "--max-target-gb", type=float, default=2.5, help="clean the work dir's target dir when it gets bigger"
    )
    args = parser.parse_args()

    all_pages = pages()
    names = args.functions or sorted(all_pages, key=lambda n: (crate_of(all_pages[n]), n))

    results = {}
    if os.path.exists(OUTPUT):
        with open(OUTPUT) as f:
            results = json.load(f)
    if not args.functions:
        results = {k: v for k, v in results.items() if k in all_pages}

    app = App(os.path.abspath(args.work_dir))
    cache = {}
    failed = []

    for i, name in enumerate(names, 1):
        print(f"[{i}/{len(names)}] {name} ...", end=" ", flush=True)
        hash = fingerprint(name, all_pages[name])
        if not args.force and results.get(name, {}).get("fingerprint") == hash:
            print(f"unchanged, {results[name]['diff']} bytes")
            continue
        try:
            result = measure(app, name, all_pages[name], cache)
        except RuntimeError as e:
            print("failed")
            print(e, file=sys.stderr)
            failed.append(name)
            continue

        if result is None:
            print("no Demo doctest, skipped")
            results.pop(name, None)
        else:
            print(f"{result['diff']} bytes")
            results[name] = {**result, "fingerprint": hash}

        with open(OUTPUT, "w") as f:
            json.dump(dict(sorted(results.items())), f, indent=2)
            f.write("\n")

        if app.target_size() > args.max_target_gb * 1e9:
            shutil.rmtree(os.path.join(app.dir, "target"))

    if failed:
        print(f"failed: {', '.join(failed)}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
