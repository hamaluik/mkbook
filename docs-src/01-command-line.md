---
title = "Command-line Interface"
---

_mkbook_ may be installed using _Cargo_ (`cargo install --force --path .` in the _mkbook_ repo directory), and after that it presents a command-line interface:

```sh
$ mkbook
mkbook 0.3.0
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
    watch    build the book and continually rebuild whenever the source changes
```

# The Init Command

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

# The Build Command

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

# The Watch Command

The watch command is basically the same as the `build` command, however after building it continues to monitor the source directory and if _any_ changes are made (a file is saved, renamed, removed, created, etc), the entire book is re-built. In the future, this will hopefully be smarter but for now it just the whole thing at once. Stop watching using <kbd>Ctrl+C</kbd> or sending `SIGINT`.

```sh
$ mkbook build --help
mkbook-watch 
build the book and continually rebuild whenever the source changes

USAGE:
    mkbook watch [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --in <in>      an optional directory to take the book sources from [default: src]
    -o, --out <out>    an optional directory to render the contents into [default: book]
```

# Sample Usages

Build the [GitHub Pages](https://pages.github.com/) document (this book):

```sh
mkbook build -i docs-src -o docs
```

Build the book, continually watching for changes and enabling auto-reloading in the browser so you can see the book update as you write:

```sh
mkbook watch -i docs-src -o docs --reload
```

Build a [LaTeX](https://www.latex-project.org/) version of the book, then compile it to a [PDF](https://en.wikipedia.org/wiki/PDF) and open it in [evince](https://wiki.gnome.org/Apps/Evince):

```sh
mkdir build
mkbook build -i docs-src -o docs --latex build/book.tex
cd build
xelatex -shell-escape book.tex
xelatex -shell-escape book.tex
evince book.pdf
```
