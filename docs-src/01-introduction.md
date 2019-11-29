---
title = "Introduction"
---

# Introduction

_mkbook_ is my simpler alternative to [_mdbook_](https://crates.io/crates/mdbook) which is a great tool, but for which I really dislike some of the decisions they took, such as relying on javascript for highlighting and navigation, and including a lot of bells and whistles such as javascript-based search.

This tool aims to work somewhat similarly to _mdbook_, but is generally intended to be a more minimal alternative that is customized more towards my needs and desires than anything else.

If you're not familiar with _mdbook_, _mkbook_ is a tool to convert a collection of [Markdown](https://commonmark.org/) files into a static website / book which can be published online. It was created to help me write documentation with minimum fuss while presenting it in an easy-to-consume manner.

## Command-line Interface

_mkbook_ may be installed using _Cargo_ (`cargo install --force --path .` in the _mkbook_ repo directory), and after that it presents a command-line interface:

```sh
$ mkbook
mkbook 0.2.0
Kenton Hamaluik <kenton@hamaluik.ca>


USAGE:
    mkbook [SUBCOMMAND]

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


SUBCOMMANDS:
    build    build the book
    help     Prints this message or the help of the given subcommand(s)
    init     initialize a mkbook directory tree
```

### The Init Command

The init command is a tool to help you get started, and will create an initial `README.md` file and a stub of your first chapter.

```sh
$ mkbook init --help
mkbook-init 
initialize a mkbook directory tree

USAGE:
    mkbook init [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --directory <directory>    an optional directory to initialize into [default: src]
```

### The Build Command

The build command is the primary command for _mkbook_, and is responsible for taking the `.md` files and building the resulting website.

```sh
$ mkbook build --help
mkbook-build 
build the book

USAGE:
    mkbook build [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --in <in>      an optional directory to take the book sources from [default: src]
    -o, --out <out>    an optional directory to render the contents into [default: book]
```
