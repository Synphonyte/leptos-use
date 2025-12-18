<br/>

<p align="center">
    <a href="https://github.com/synphonyte/leptos-use">
        <img src="https://raw.githubusercontent.com/synphonyte/leptos-use/main/docs/logo.svg" alt="Leptos-Use – Collection of essential Leptos utilities" width="150"/>
    </a>
</p>

<h4 align="center">Collection of essential Leptos utilities</h4>
<p align="center">Inspired by React-Use / VueUse</p>

<p align="center">
    <a href="https://crates.io/crates/leptos-use"><img src="https://img.shields.io/crates/v/leptos-use.svg?label=&color=%232C1275" alt="Crates.io"/></a>
    <a href="https://leptos-use.rs/server_side_rendering.html"><img src="https://img.shields.io/badge/-SSR-%236a214b" alt="SSR"></a> 
    <a href="https://leptos-use.rs"><img src="https://img.shields.io/badge/-docs%20%26%20demos-%239A233F" alt="Docs & Demos"></a> 
    <a href="https://leptos-use.rs"><img src="https://img.shields.io/badge/-89%20functions-%23EF3939" alt="89 Functions" /></a>
</p>

<br/>
<br/>
<br/>

## Usage

![Crates.io Total Downloads](https://img.shields.io/crates/d/leptos-use)
[![Docs](https://docs.rs/leptos-use/badge.svg)](https://docs.rs/leptos-use/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/synphonyte/leptos-use#license)
[![Build Status](https://github.com/synphonyte/leptos-use/actions/workflows/cd.yml/badge.svg)](https://github.com/synphonyte/leptos-use/actions/workflows/cd.yml)
[![Discord](https://img.shields.io/discord/1031524867910148188?color=%237289DA&label=discord)](https://discord.com/channels/1031524867910148188/1121154537709895783)

```rust
use leptos::prelude::*;
use leptos_use::{use_mouse, UseMouseReturn};

#[component]
fn Demo() -> impl IntoView {
    let UseMouseReturn { x, y, .. } = use_mouse();

    view! { cx,
        {x} " x " {y}
    }
}
```

Missing a function? Open a ticket or PR!

## Development

To run all tests run

```shell
# Run tests (general)
cargo test --features math,docs,ssr

# Run tests (axum) use_cookie
cargo test --features math,docs,ssr,axum --doc use_cookie

# Run tests (axum) use_locale
cargo test --features math,docs,ssr,axum --doc use_locale

# Run tests (actix) use_cookie
cargo test --features math,docs,ssr,actix --doc use_cookie

# Run tests (actix) use_locale
cargo test --features math,docs,ssr,actix --doc use_locale
```

### Book

First you need to install

```shell
cargo install mdbook mdbook-cmdrun trunk
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
python3 post_build.py use_storage
```

### New Function Template

To scaffold a new function quickly you can run `template/createfn.sh`. It requires
that [`ffizer`](https://ffizer.github.io/) and Python 3 is installed.
This will create the function file in the src directory, scaffold an example directory and an entry in the book.

## Leptos compatibility

| Crate version | Compatible Leptos version |
|---------------|---------------------------|
| <= 0.3        | 0.3                       |
| 0.4, 0.5, 0.6 | 0.4                       |
| 0.7, 0.8, 0.9 | 0.5                       |
| 0.10 – 0.13   | 0.6                       |
| 0.14, 0.15    | 0.7                       |
| 0.16, 0.17    | 0.8                       |
