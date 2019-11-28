---
title = "Introduction"
---

_mkbook_ is my simpler alternative to [_mdbook_](https://crates.io/crates/mdbook) which is a great tool, but for which I really dislike some of the decisions they took, such as relying on javascript for highlighting and navigation, and including a lot of bells and whistles such as javascript-based search.

This tool aims to work somewhat similarly to _mdbook_, but is generally intended to be a more minimal alternative that is customized more towards my needs and desires than anything else.

_mkbook_ may be installed using _Cargo_ (`cargo install --force --path .` in the _mkbook_ repo directory), and after that it presents a command-line interface:

```
$ mkbook
mkbook 0.1.0
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

Currently, only the `build` subcommand does anything (it builds your book!), but this functionality is still WIP:

```
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
