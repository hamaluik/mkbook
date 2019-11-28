# mkbook

_mkbook_ is my simpler alternative to [mdbook](https://crates.io/crates/mdbook) which is a great tool, but for which I really dislike some of the decisions they took, such as relying on javascript for highlighting and navigation, and including a lot of bells and whistles such as javascript-based search.

This tool aims to work somewhat similarly to _mdbook_, but is generally intended to be a more minimal alternative that is customized more towards my needs and desires than anything else.

Still very WIP, but it can convert `.md` files into fancy-looking `.html` files, demo it by building the _mkbook_ book by running: `cargo run -- build -i docs-src -o docs` and then serving the `docs` directory. Alternatively, view these generated docs [here](https://hamaluik.github.io/mkbook/).
