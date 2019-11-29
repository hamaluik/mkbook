---
title = "Front Matter"
---

# Front Matter

Each `.md` file can optionally contain a header with metadata describing the document. If the header isn't present, or if any keys are missing, default values will be used.

To insert a header into a `.md` file, insert three dashes (`---`), followed by a new-line, followed by the front matter contents, followed by a newline, then another three dashes and a new-line. The metadata is in the [TOML](https://github.com/toml-lang/toml) format, so for example the front-matter (and first line) for a file could look like this:

```md
---
title = "Front Matter"
author = "Kenton Hamaluik"
pubdate = 2019-11-29T15:22:00-07:00
---

# Front Matter

Each `.md` file can optionally contain a header with metadata describing the document. If the header isn't present, or if any keys are missing, default values will be used.
```

## Supported Keys

The list of supported keys is subject to change, but for now it is as follows:

title

: A human-readable title for the document (defaults to the filename)

author

: The author (or authors) who wrote the chapter (defaults to "Anonymous")

pubdate

: The [RFC 3339](http://tools.ietf.org/html/rfc3339) timestamp of when the chapter was published (defaults to the time at build)

url

: The relative URL of the file, defaults to the generated route (you probably shouldn't set this one)
