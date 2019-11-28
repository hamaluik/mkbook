---
title = "Front Matter"
---

# Front Matter

Each `.md` file can optionally contain a header with metadata describing the document. If the header isn't present, default values will be used which may look ugly.

To insert a header into a `.md` file, insert three dashes (`---`), followed by a new-line, followed by the front matter contents, followed by a newline, then another three dashes and a new-line. The metadata is in the [TOML](https://github.com/toml-lang/toml) format, so for example the front-matter (and first line) for this file looks like:

```md
---
title = "Front Matter"
---

# Front Matter

Each `.md` file can optionally contain a header with metadata describing the document. If the header isn't present, default values will be used which may look ugly.
```

## Supported Keys

The list of supported keys is subject to change, but for now it is as follows:

title

: A human-readable title for the document

