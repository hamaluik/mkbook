---
title = "Structure"
---

_mkbook_ currently only supports two types of assets to use in rendering: assets (images, etc), and documents (markdown files).

## Assets

```rust
unimplemented!()
```

## Documents

For now, _mkbook_ only works on a flat list of markdown files, with the intent of each markdown file being its own chapter. Subdirectories and files that don't end in a `.md` extension are completely ignored. The order of the book is based on the alphabetical order of the file names (actually it's based on Rust's [implementation of `PartialOrd` for str](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#impl-PartialOrd%3Cstr%3E)). Thus, it is recommended to lay out your book chapters with manual numbering of the file names, as such:

```
src/
├── 00-foreword.md
├── 01-introduction.md
├── 02-my-first-chapter.md
└── etc...
```

An index and navigation will be automatically generated from these files, taking the information for each file from it's front-matter.
