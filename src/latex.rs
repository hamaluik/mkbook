use std::path::{Path, PathBuf};
use askama::Template;
use std::fs;
use comrak::ComrakOptions;

mod filters;

use super::models::chapter::Chapter;
use super::models::frontmatter::FrontMatter;

lazy_static! {
    static ref COMRAK_OPTIONS: ComrakOptions = ComrakOptions {
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
}

#[derive(Template)]
#[template(path = "book.tex", escape = "none")]
struct BookTemplate<'a, 'b, 'c> {
    front: &'a FrontMatter,
    description: &'b str,
    chapters: Vec<&'c Chapter>,
}

fn format_text<'a>(node: &'a comrak::nodes::AstNode<'a>, output: &mut String) {
    use comrak::nodes::NodeValue;
    match &node.data.borrow().value {
        NodeValue::Text(text) => {
            if let Ok(text) = std::str::from_utf8(text) {
                output.push_str(text);
            }
        },
        NodeValue::Code(text) => {
            if let Ok(text) = std::str::from_utf8(text) {
                output.push_str("\\texttt{");
                output.push_str(text);
                output.push_str("}");
            }
        },
        _ => for child in node.children() { format_text(child, output); },
    }
}

fn format_node<'a>(node: &'a comrak::nodes::AstNode<'a>, output: &mut String) {
    use comrak::nodes::NodeValue;
    match &node.data.borrow().value {
        NodeValue::Document => for child in node.children() { format_node(child, output); },
        NodeValue::BlockQuote => {
            output.push_str("\\begin{quote}\n");
            for child in node.children() { format_node(child, output); }
            output.push_str("\\end{quote}\n");
        },
        NodeValue::List(node_list) => {
            match node_list.list_type {
                comrak::nodes::ListType::Bullet => output.push_str("\\begin{itemize}\n"),
                comrak::nodes::ListType::Ordered => output.push_str("\\begin{enumerate}\n"),
            }
            for child in node.children() { format_node(child, output); }
            match node_list.list_type {
                comrak::nodes::ListType::Bullet => output.push_str("\\end{itemize}\n"),
                comrak::nodes::ListType::Ordered => output.push_str("\\end{enumerate}\n"),
            }
        },
        NodeValue::Item(_) => {
            output.push_str("\\item ");
            for child in node.children() { format_node(child, output); }
            output.push_str("\n");
        },
        NodeValue::DescriptionList => {
            output.push_str("\\begin{description}\n");
            for child in node.children() { format_node(child, output); }
            output.push_str("\\end{description}\n");
        },
        NodeValue::DescriptionItem(_) => for child in node.children() { format_node(child, output); },
        NodeValue::DescriptionTerm => {
            output.push_str("\\item [");
            let mut term: String = String::default();
            for child in node.children() { format_text(child, &mut term); }
            output.push_str(term.trim());
            output.push_str("] ");
        },
        NodeValue::DescriptionDetails => {
            for child in node.children() { format_node(child, output); }
            output.push_str("\n");
        },
        NodeValue::CodeBlock(node_code_block) => {
            
        },
        NodeValue::HtmlBlock(node_html_block) => {
            
        },
        NodeValue::Paragraph => {
            for child in node.children() {
                format_node(child, output);
            }
            output.push_str("\n\n");
        },
        NodeValue::Heading(node_heading) => {
            match node_heading.level {
                1 => output.push_str("\\section{"),
                2 => output.push_str("\\subsection{"),
                3 => output.push_str("\\subsubsection{"),
                4 => output.push_str("\\paragraph{"),
                5 => output.push_str("\\subparagraph{"),
                6 => output.push_str("\\textbf{"),
                _ => unreachable!()
            }
            for child in node.children() { format_text(child, output); }
            output.push_str("}");
        },
        NodeValue::ThematicBreak => {
            output.push_str("\\hline\n");
        },
        NodeValue::FootnoteDefinition(text) => {
            
        },
        NodeValue::Table(table_alignments) => {
            
        },
        NodeValue::TableRow(bool) => {
            
        },
        NodeValue::TableCell => {
            
        },
        NodeValue::Text(text) => {
            if let Ok(text) = std::str::from_utf8(text) {
                output.push_str(text);
            }
        },
        NodeValue::TaskItem(bool) => {
            
        },
        NodeValue::SoftBreak => {
            output.push_str("\n");
        },
        NodeValue::LineBreak => {
            output.push_str("\\newline");
        },
        NodeValue::Code(text) => {
            if let Ok(text) = std::str::from_utf8(text) {
                output.push_str("\\verb|");
                output.push_str(text);
                output.push_str("|");
            }
        },
        NodeValue::HtmlInline(text) => {
            
        },
        NodeValue::Emph => {
            output.push_str("\\emph{");
            for child in node.children() { format_node(child, output); }
            output.push_str("}");
        },
        NodeValue::Strong => {
            output.push_str("\\textbf{");
            for child in node.children() { format_node(child, output); }
            output.push_str("}");
        },
        NodeValue::Strikethrough => {
            output.push_str("\\sout{");
            for child in node.children() { format_node(child, output); }
            output.push_str("}");
        },
        NodeValue::Superscript => {
            output.push_str("\\textsuperscript{");
            for child in node.children() { format_node(child, output); }
            output.push_str("}");
        },
        NodeValue::Link(node_link) => {
            
        },
        NodeValue::Image(node_link) => {
            
        },
        NodeValue::FootnoteReference(text) => {
            
        },
    }
}

fn format_markdown(src: &str) -> Result<String, Box<dyn std::error::Error>> {
    let arena = comrak::Arena::new();
    let root = comrak::parse_document(
        &arena,
        src,
        &COMRAK_OPTIONS);

    let mut latex: String = String::with_capacity(src.len());
    format_node(&root, &mut latex);

    Ok(latex)
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

    // load the book
    let mut book = super::load_book(&src)?;

    // then convert all the markdown
    book.description = format_markdown(&book.description)?;
    for chapter in book.chapters.iter_mut() {
        chapter.contents = format_markdown(&chapter.contents)?;
    }

    // and render to a template
    let latexbook = BookTemplate {
        front: &book.front,
        description: &book.description,
        chapters: book.chapters.iter().collect(),
    };
    let rendered = latexbook.render()?;
    fs::write(dest, rendered)?;

    Ok(())
}