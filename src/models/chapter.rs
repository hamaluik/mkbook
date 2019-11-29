use std::path::PathBuf;
use super::frontmatter::FrontMatter;

#[derive(Debug)]
pub struct Chapter {
    pub url: String,
    pub front: FrontMatter,
    pub sections: Vec<Chapter>,
    pub source: PathBuf,
    pub contents: String,
}
