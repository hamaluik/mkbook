---
title = "CommonMark"
---

# CommonMark

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
