---
title = "Structure"
---

# Structure

_mkbook_ follows a fairly simple directory structure for now, with a `README.md` file declaring the book's metadata, and `.md` files defining each chapter of the book.

## `README.md`

_mkbook_ generally requires a `README.md` file to reside in your source directory. This file is responsible for defining the metadata associated with your book:

* The book's title (`title`)
* The book's author (`author`)
* The publication date (`pubdate`)
* The canonical URL for the book (`url`)
* A markdown-formatted description of the book

If the `README.md` file or any of the entries are missing, default values will be used. The `README.md` file should be formatted as any other page, with the `title`, `author`, `pubdate`, and `url` specified in the frontmatter, and the book description the _Markdown_ contents of the `README.md` file.

### Sample

```md
---
title = "The mkbook Book"
author = "Kenton Hamaluik"
url = "https://hamaluik.github.io/mkbook/"
---

_mkbook_ is my simpler alternative to [mdbook](https://crates.io/crates/mdbook)
which is a great tool, but for which I really dislike some of the decisions they
took, such as relying on javascript for highlighting and navigation, and
including a lot of bells and whistles such as javascript-based search.

This tool aims to work somewhat similarly to _mdbook_, but is generally intended
to be a more minimal alternative that is customized more towards my needs and
desires than anything else.
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

Any files in the `src` directory which are not included in `.gitignore` and do not end in the extension `.md` will be copied to the output folder. You can use this to include images, files, etc, for example the following image is an asset bundled with the book:

```md
![chapter-six](book-chapter-six-5834.jpg "Photo by Kaboompics.com from Pexels")
```

![chapter-six](../book-chapter-six-5834.jpg "Photo by Kaboompics.com from Pexels")


## Documents

_mkbook_ works on mostly a flat directory structure, however one level of sub-directories are supported in order to create sections within chapters. Files that don't end in a `.md` extension are completely ignored. Each `.md` file in the root source directly is it's own chapter. To create chapters with sub-sections, create a sub-directory in the root directory and then create a `README.md` file, which will become the root of the chapter, with all `.md` files in the sub-directory becoming sections in the chapter. The `title` in the `README.md` file's frontmatter will be used as the name of the chapter.

The order of the book is based on the alphabetical order of the file names (actually it's based on Rust's [implementation of `PartialOrd` for str](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#impl-PartialOrd%3Cstr%3E)). Thus, it is recommended to lay out your book chapters with manual numbering of the file names, as such:

```
src/
├── README.md
├── 00-foreword.md
├── 01-introduction.md
├── my-picture.jpg
└── 02-my-first-chapter
    ├── README.md
    ├── 01-my-first-section.md
    ├── 02-my-second-section.md
    └── etc...
```

An index and navigation will be automatically generated from these files, taking the information for each file from it's front-matter.
