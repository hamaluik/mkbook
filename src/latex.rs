use std::path::{Path, PathBuf};
use askama::Template;
use std::fs;

mod filters;

use super::models::frontmatter::FrontMatter;

#[derive(Template)]
#[template(path = "book.tex", escape = "none")]
struct BookTemplate<'a> {
    book: &'a FrontMatter,
}

pub fn build<PIn: AsRef<Path>, POut: AsRef<Path>>(src: PIn, dest: POut) -> Result<(), Box<dyn std::error::Error>> {
    let src = PathBuf::from(src.as_ref());
    let dest = PathBuf::from(dest.as_ref());
    if let Some(parent) = dest.parent() {
        if !parent.exists() {
            fs::create_dir_all(&parent)?;
            log::info!("created directory `{}`...", parent.display());
        }
    }

    // load our book
    let book_readme_path = src.join("README.md");
    let (book_front, book_description) = if book_readme_path.exists() {
        let contents = fs::read_to_string(&book_readme_path)?;
        let (front, contents) = super::extract_frontmatter(&contents)?;
        (front, contents)
    }
    else {
        let content = String::new();
        (None, content)
    };
    let book_front = FrontMatter::from_root(book_front.unwrap_or_default());

    let book: BookTemplate = BookTemplate {
        book: &book_front,
    };

    let rendered = book.render()?;
    std::fs::write(dest, rendered)?;

    Ok(())
}