---
title = "How it Works"
---

_mkbook_ generates a completely static, javascript-free website from a series of Markdown files. All of the layout and styling is controlled purely by hand-crafted CSS specific to this book's purpose.

## Assets

_mkbook_ currently bundles two assets which get written into the book directory: `favicon.ico`, and `icons.svg`. `favicon.ico` is the [Font Awesome 5 book icon](https://fontawesome.com/icons/book?style=solid), and `icons.svg` contains 3 [Font Awesome 5](https://fontawesome.com/) arrow icons: [arrow-left](https://fontawesome.com/icons/arrow-left?style=solid), [arrow-right](https://fontawesome.com/icons/arrow-right?style=solid), and [arrow-up](https://fontawesome.com/icons/arrow-up?style=solid) which are used for navigation. These files are compiled into the _mkbook_ binary using the [`include_bytes!` macro](https://doc.rust-lang.org/std/macro.include_bytes.html), and written to the output folder on each build.

## Styling

_mkbook_ utilizes [Sass](https://sass-lang.com/) to define it's styles; you can view the sources [on github](https://github.com/hamaluik/mkbook/tree/master/style). In _mkbook_'s build script, the styles are compiled from their native `.scss` format into a single, compressed `.css` file using [sass-rs](https://crates.io/crates/sass-rs). The resulting `.css` file is then bundled into the binary using the [`include_str!` macro](https://doc.rust-lang.org/std/macro.include_str.html). When a book is generated, this `.css` is written to the output folder as `style.css`, where it is included by each generated `.html` file.

## Templates

_mkbook_ contains two template files: one for the index, and one for each page / chapter, and uses [Askama](https://crates.io/crates/askama) to render the templates. Since the _Askama_ templates are compiled when _mkbook_ is compiled, it is not currently possible to change the templates at run time. You can view the sources for these templates [on github](https://github.com/hamaluik/mkbook/tree/master/templates).

## Markdown Formatting

Markdown is formatted usiing [comrak](https://crates.io/crates/comrak) with some specific options, see the [Markdown chapter](02-markdown.html) for more information.

## Syntax Highlighting

Code is syntax-highlighted using [syntect](https://crates.io/crates/syntect) with the default langauges and the `base16-eighties` colour scheme.
