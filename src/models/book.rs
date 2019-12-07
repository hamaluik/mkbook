use super::frontmatter::FrontMatter;
use super::chapter::Chapter;

pub struct Book {
    pub front: FrontMatter,
    pub description: String,
    pub chapters: Vec<Chapter>,
}