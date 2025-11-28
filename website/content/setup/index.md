+++
title = "Setup"
+++

<!-- toc -->

## Installing Rust

Before installing Clings, you must have the **latest version of Rust** installed.
Visit [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for further instructions.
This will also install _Cargo_, Rust's package/project manager.

> 🐧 If you are on **Linux**, make sure you have `gcc` installed (_for a linker_).
>
> Debian: `sudo apt install gcc`\
> Fedora: `sudo dnf install gcc`

> 🍎 If you are on **MacOS**, make sure you have _Xcode and its developer tools_ installed: `xcode-select --install`

## Installing Clings

The following command will download and compile Clings:

```bash
cargo install clings
```

{% details(summary="If the installation fails…") %}

- Make sure you have the latest Rust version by running `rustup update`
- Try adding the `--locked` flag: `cargo install clings --locked`
- Otherwise, please [report the issue](https://github.com/rust-lang/clings/issues/new)

{% end %}

## Initialization

After installing Clings, run the following command to initialize the `clings/` directory:

```bash
clings init
```

{% details(summary="If the command <code>clings</code> can't be found…") %}

You are probably using Linux and installed Rust using your package manager.

Cargo installs binaries to the directory `~/.cargo/bin`.
Sadly, package managers often don't add `~/.cargo/bin` to your `PATH` environment variable.

- Either add `~/.cargo/bin` manually to `PATH`
- Or uninstall Rust from the package manager and [install it using the official way with `rustup`](https://www.rust-lang.org/tools/install)

{% end %}

Now, go into the newly initialized directory and launch Clings for further instructions on getting started with the exercises:

```bash
cd clings/
clings
```

## Working environment

### Editor

Our general recommendation is [VS Code](https://code.visualstudio.com/) with the [rust-analyzer plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
But any editor that supports [rust-analyzer](https://rust-analyzer.github.io/) should be enough for working on the exercises.

### Terminal

While working with Clings, please use a modern terminal for the best user experience.
The default terminal on Linux and Mac should be sufficient.
On Windows, we recommend the [Windows Terminal](https://aka.ms/terminal).

## Usage

After being done with the setup, visit the [**usage**](@/usage/index.md) page for some info about using Clings 🚀
