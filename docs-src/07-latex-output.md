---
title = "LaTeX Output"
---

_mkbook_ can also export a [LaTeX](https://www.latex-project.org/) file which can be used to convert your book to a beatiful, ready-to-print [PDF](https://en.wikipedia.org/wiki/PDF). This feature is still under heavy development as it's not quite as smooth as I would like, and the generated `.tex` document is perhaps a bit too customized—I'm still exploring this.

For now, however, you can convert your book into a single `.tex` file with the following command which will create the file `./print/book.tex` along with any images needed to render the book:

```sh
mkbook build -l ./print/book.tex
```

Note that this command is more about preparing a `.tex` file that you can then further customize for your own book than having a complete, ready-to-go PDF that is entirely your own—the current LaTeX template that gets generated works for me but it may not work for you.

# Images

If an image in the document is an external image (i.e. it starts with `http://` or `https://`), _mkbook_ will attempt to download the image the same directory that the generated LaTeX document resides in. If it cannot do so, it will tell you. If, on the other hand, the image is in the source tree, it will be copied over the same way that any other asset is and should be available to the LaTeX file.

Similar to this, _mkbook_ will attempt to render any `plantuml` code sections into `.svg` files which also get included in the book.

# Building the Book

The current LaTeX template requires the following packages to be installed:

* [ulem](https://ctan.org/pkg/ulem)
* [fontspec](https://ctan.org/pkg/fontspec)
* [sectsty](https://ctan.org/pkg/sectsty)
* [xcolor](https://ctan.org/pkg/xcolor)
* [minted](https://ctan.org/pkg/minted)
* [amsmath](https://ctan.org/pkg/amsmath)
* [amssymb](https://ctan.org/pkg/amssymb)
* [enumitem](https://ctan.org/pkg/enumitem)
* [textcomp](https://ctan.org/pkg/textcomp)
* [graphicx](https://ctan.org/pkg/graphicx)
* [float](https://ctan.org/pkg/float)
* [svg](https://ctan.org/pkg/svg)

The template also requires [XeTeX](https://www.tug.org/xetex/) and the following fonts to be available on your system:

* [Crimson](https://github.com/skosch/Crimson)
* [Poppins](https://www.fontsquirrel.com/fonts/poppins)
* [Source Code Pro](https://github.com/adobe-fonts/source-code-pro)

Finally, in order to color the source code, you must have [Pygments](https://pygments.org/) installed and the `pygmentize` executable must be available on your path.

If you meet all these requirements, you can build the book using `xelatex`. Assuming you built the `book.tex` file in the `print` directory as above:

```sh
cd print
xelatex -shell-escape book.tex
xelatex -shell-escape book.tex
```

Note that the `-shell-escape` argument is required in order to get _Pygments_ to colour your source code, and the `xelatex` command is run twice in order to properly build the table of contents.

Note also that in the current template, the pages that are created are 5.5 inches by 8 inches. This is to facilitate booklet printing on North American letter paper. Feel free to change this in the generated `book.tex` file before compiling if you need to.

## Compiling a Booklet

If you want to easily print this book as a booklet, you can take one more step to arrange the pages so that a simple duplex print on any printer will produce signatures that you can easily bind yourself (there are many tutorials online for doing this, I recommend [Easy paperback book binding how-to](https://mostlymaths.net/2009/04/easy-paperback-book-binding-how-to.html/) by Rubén Berenguel).

The first step is to create a file alongside your compiled `book.pdf` file called `printbook.tex` with the contents as such:

```latex
\documentclass[letterpaper]{article}
\usepackage[final]{pdfpages}
\begin{document}
\includepdf[pages=-,nup=1x2,landscape,signature=32]{book.pdf}
\end{document}
```

You can change the value of `signature` as you like, but keep it a multiple of 4. The [signature](https://en.wikipedia.org/wiki/Section_(bookbinding)) is the number of pages (**not** sheets of paper) which get combined into a "mini-booklet", and the final book is a combination of all of the signatures ("mini-booklets") to make the full book. Essentially, if you divide this number by 4, you'll get the number of sheets of paper that you'll have to staple together at a time. For a signature of 32 pages, this will mean stapling together 8 pages at a time.

Note that if you have a relatively short book, it may be advantageous to just do all of the book's pages into one signature, in this case make the signature the next multiple-of-four value higher than the total number of sheets in the `book.pdf` file. For example: if `book.pdf` contains 45 pages, make `signature=48` to put everything into a single signature.

Finally, compile `printbook.tex` using `pdflatex`:

```sh
pdflatex printbook.tex
```

As a sample, you can view the compiled [book](book.pdf) and [printbook](printbook.pdf) files for this book to see how this can turn out.
