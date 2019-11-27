# mkbook

**mkbook** is my simpler alternative to [mdbook](https://crates.io/crates/mdbook) which is a great tool, but for which I really dislike some of the decisions they took, such as relying on javascript, etc.

This tool aims to work somewhat similarly to _mkbook_, but is generally intended to be a more minimal alternative that is customized more towards my needs and desires than anything else.

Still very WIP, but it can convert `.md` files into fancy-looking `.html` files, demo it by running: `cargo run -- build demo` and then serving the `demo/book` directory.
