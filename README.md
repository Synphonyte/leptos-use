<p align="center">
    <a href="https://github.com/synphonyte/leptos-use">
        <img src="https://raw.githubusercontent.com/synphonyte/leptos-use/main/docs/logo.svg" alt="Leptos-Use – Collection of essential Leptos utilities" width="150"/>
    </a>
</p>

<h4 align="center">Collection of essential Leptos utilities</h4>
<p align="center">Inspired by React-Use / VueUse / SolidJS-USE</p>

<p align="center">
    <a href="https://leptos-use.rs"><b>Guide</b></a> • 
    <a href="https://docs.rs/leptos-use"><b>Docs.rs</b></a> •
    <a href="https://crates.io/crates/leptos-use"><b>Crates.io</b></a>
</p>

-----

[![Crates.io](https://img.shields.io/crates/v/leptos-use.svg)](https://crates.io/crates/leptos-use)
[![Docs](https://docs.rs/leptos-use/badge.svg)](https://docs.rs/leptos-use/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/synphonyte/leptos-use#license)
[![Build Status](https://github.com/synphonyte/leptos-use/actions/workflows/ci.yml/badge.svg)](https://github.com/synphonyte/leptos-use/actions/workflows/ci.yml)

We have only just begun implementing the first dozen functions but they are already very usable and ergonomic.

Missing a function? Open a ticket or PR!

## Development

To run all tests run

```shell
cargo test --all-features
```

## Book

First you need to install

```shell
cargo install mdbook-cmdrun trunk
```

To build the book go in your terminal into the docs/book folder
and run

```shell
mdbook serve
```

This builds the html version of the book and runs a local dev server.
To also add in the examples open another shell and run

```shell
python3 post_build.py
```

If you only want to add the example for one function you can run for example

```shell
python3 post_build.py use_media_query
```