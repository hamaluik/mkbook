---
title = "Structure"
---

_mkbook_ follows a fairly simple directory structure for now, with a `mkbook.toml` file declaring the book's metadata, and `.md` files defining each chapter of the book.

## `mkbook.toml`

_mkbook_ generally requires a `mkbook.toml` file to reside in your source directory. This file is responsible for defining the metadata associated with your book:

* The book's title (`title`)
* The book's author (`author`)
* The publication date (`pubdate`)
* The canonical URL for the book (`url`)
* A markdown-formatted description of the book (`description`)

If the `mkbook.toml` file or any of the entries are missing, default values will be used.

### Sample

```toml
title = "The mkbook Book"
author = "Kenton Hamaluik"
url = "https://hamaluik.github.io/mkbook/"
description = """
_mkbook_ is my simpler alternative to [mdbook](https://crates.io/crates/mdbook) which is a great tool, but for which I really dislike some of the decisions they took, such as relying on javascript for highlighting and navigation, and including a lot of bells and whistles such as javascript-based search.

This tool aims to work somewhat similarly to _mdbook_, but is generally intended to be a more minimal alternative that is customized more towards my needs and desires than anything else.
"""
```

### Default Values

`title`

: "My Cool Book"

`author`

: "Anonymous"

`pubdate`

: The date the book was built from the command line, in UTC time

`url`

: ""

`description`

: ""

## Assets

```rust
unimplemented!()
```

## Documents

For now, _mkbook_ only works on a flat list of markdown files, with the intent of each markdown file being its own chapter. Subdirectories and files that don't end in a `.md` extension are completely ignored. The order of the book is based on the alphabetical order of the file names (actually it's based on Rust's [implementation of `PartialOrd` for str](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#impl-PartialOrd%3Cstr%3E)). Thus, it is recommended to lay out your book chapters with manual numbering of the file names, as such:

```
src/
├── mkbook.toml
├── 00-foreword.md
├── 01-introduction.md
├── 02-my-first-chapter.md
└── etc...
```

An index and navigation will be automatically generated from these files, taking the information for each file from it's front-matter.
