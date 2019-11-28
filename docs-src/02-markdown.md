---
title = "Markdown"
---

_mkbook_ nominally utilizes [CommonMark](https://commonmark.org/) with some [GFM](https://github.github.com/gfm/) extensions through the use of the [comrak](https://crates.io/crates/comrak) crate. In using _comrak_, a specific set of options are used, which are listed here:

```rust
let options: ComrakOptions = ComrakOptions {
    hardbreaks: false,
    smart: true,
    github_pre_lang: false,
    default_info_string: None,
    unsafe_: true,
    ext_strikethrough: true,
    ext_tagfilter: false,
    ext_table: true,
    ext_autolink: true,
    ext_tasklist: true,
    ext_superscript: true,
    ext_header_ids: Some("header".to_owned()),
    ext_footnotes: true,
    ext_description_lists: true,
    ..ComrakOptions::default()
};
```

Mostly, know that the following extensions are enabled:

* [Strikethrough](https://github.github.com/gfm/#strikethrough-extension-)
* [Tables](https://github.github.com/gfm/#tables-extension-)
* [Autolinks](https://github.github.com/gfm/#autolinks-extension-)
* [Task Lists](https://github.github.com/gfm/#task-list-items-extension-)
* Superscripts (`e = mc^2^.` → `e = mc<sup>2</sup>.`)
* [Footnotes](https://kramdown.gettalong.org/syntax.html#footnotes)
* Description Lists:
  ```md
  First term
  
  : Details for the **first term**
  
  Second term
  
  : Details for the **second term**
  
      More details in second paragraph.
  ```

## Syntax Highlight

GFM syntax highlighting is also available by using fenced code tags with a label denoting the language, as such:

~~~md
```c++
#include <stdio>

int main() {
    std::cout << "Hello, world!" << std::endl;
    return 0;
}
```
~~~

which results in:

```c++
#include <stdio>

int main() {
    std::cout << "Hello, world!" << std::endl;
    return 0;
}
```

To denote the language you can either use one the language's extensions as the label, or the full name of the language (which is **not** case-sensitive).

The list of supported languages is currently as follows:

| Language Name | Supported Tags / Extensions |
|:-|:-|
| ASP | `asa` |
| ActionScript | `as` |
| AppleScript | `applescript`, `script editor` |
| Batch File | `bat`, `cmd` |
| BibTeX | `bib` |
| Bourne Again Shell (bash) | `sh`, `bash`, `zsh`, `fish`, `.bash_aliases`, `.bash_completions`, `.bash_functions`, `.bash_login`, `.bash_logout`, `.bash_profile`, `.bash_variables`, `.bashrc`, `.profile`, `.textmate_init` |
| C | `c`, `h` |
| C# | `cs`, `csx` |
| C++ | `cpp`, `cc`, `cp`, `cxx`, `c++`, `C`, `h`, `hh`, `hpp`, `hxx`, `h++`, `inl`, `ipp` |
| CSS | `css`, `css.erb`, `css.liquid` |
| Cargo Build Results | |
| Clojure | `clj` |
| D | `d`, `di` |
| Diff | `diff`, `patch` |
| Erlang | `erl`, `hrl`, `Emakefile`, `emakefile` |
| Go | `go` |
| Graphviz (DOT) | `dot`, `DOT`, `gv` |
| Groovy | `groovy`, `gvy`, `gradle` |
| HTML (ASP) | `asp` |
| HTML (Erlang) | `yaws` |
| HTML (Rails) | `rails`, `rhtml`, `erb`, `html.erb` |
| HTML (Tcl) | `adp` |
| HTML | `html`, `htm`, `shtml`, `xhtml`, `inc`, `tmpl`, `tpl` |
| Haskell | `hs` |
| JSON | `json`, `sublime-settings`, `sublime-menu`, `sublime-keymap`, `sublime-mousemap`, `sublime-theme`, `sublime-build`, `sublime-project`, `sublime-completions`, `sublime-commands`, `sublime-macro`, `sublime-color-scheme` |
| Java Properties | `properties` |
| Java Server Page (JSP) | `jsp` |
| Java | `java`, `bsh` |
| JavaDoc | |
| JavaScript (Rails) | `js.erb` |
| JavaScript | `js`, `htc` |
| LaTeX Log | |
| LaTeX | `tex`, `ltx` |
| Lisp | `lisp`, `cl`, `clisp`, `l`, `mud`, `el`, `scm`, `ss`, `lsp`, `fasl` |
| Literate Haskell | `lhs` |
| Lua | `lua` |
| MATLAB | `matlab` |
| Make Output | |
| Makefile | `make`, `GNUmakefile`, `makefile`, `Makefile`, `OCamlMakefile`, `mak`, `mk` |
| Markdown | `md`, `mdown`, `markdown`, `markdn` |
| MultiMarkdown | |
| NAnt Build File | `build` |
| OCaml | `ml`, `mli` |
| OCamllex | `mll` |
| OCamlyacc | `mly` |
| Objective-C | `m`, `h` |
| Objective-C++ | `mm`, `M`, `h` |
| PHP Source | |
| PHP | `php`, `php3`, `php4`, `php5`, `php7`, `phps`, `phpt`, `phtml` |
| Pascal | `pas`, `p`, `dpr` |
| Perl | `pl`, `pm`, `pod`, `t`, `PL` |
| Plain Text | `txt` |
| Python | `py`, `py3`, `pyw`, `pyi`, `pyx`, `pyx.in`, `pxd`, `pxd.in`, `pxi`, `pxi.in`, `rpy`, `cpy`, `SConstruct`, `Sconstruct`, `sconstruct`, `SConscript`, `gyp`, `gypi`, `Snakefile`, `wscript` |
| R Console | |
| R | `R`, `r`, `s`, `S`, `Rprofile` |
| Rd (R Documentation) | `rd` |
| Regular Expression | `re` |
| Regular Expressions (Javascript) | |
| Regular Expressions (Python) | |
| Ruby Haml | `haml`, `sass` |
| Ruby on Rails | `rxml`, `builder` |
| Ruby | `rb`, `Appfile`, `Appraisals`, `Berksfile`, `Brewfile`, `capfile`, `cgi`, `Cheffile`, `config.ru`, `Deliverfile`, `Fastfile`, `fcgi`, `Gemfile`, `gemspec`, `Guardfile`, `irbrc`, `jbuilder`, `podspec`, `prawn`, `rabl`, `rake`, `Rakefile`, `Rantfile`, `rbx`, `rjs`, `ruby.rail`, `Scanfile`, `simplecov`, `Snapfile`, `thor`, `Thorfile`, `Vagrantfile` |
| Rust | `rs` |
| SQL (Rails) | `erbsql`, `sql.erb` |
| SQL | `sql`, `ddl`, `dml` |
| Scala | `scala`, `sbt` |
| Shell-Unix-Generic | |
| Tcl | `tcl` |
| TeX | `sty`, `cls` |
| Textile | `textile` |
| XML | `xml`, `xsd`, `xslt`, `tld`, `dtml`, `rss`, `opml`, `svg` |
| YAML | `yaml`, `yml`, `sublime-syntax` |
| camlp4 | |
| commands-builtin-shell-bash | |
| reStructuredText | `rst`, `rest` |
