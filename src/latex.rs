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

fn escape_text(text: &str) -> String {
    text
    .replace(r"\", r"\textbackslash{}")
        .replace(r"#", r"\#")
        .replace(r"$", r"\$")
        .replace(r"%", r"\%")
        .replace(r"&", r"\&")
        .replace(r"{", r"\{")
        .replace(r"}", r"\}")
        .replace(r"^", r"\textasciicircum{}")
        .replace(r"~", r"\textasciitilde{}")
}

fn format_text<'a>(node: &'a comrak::nodes::AstNode<'a>, output: &mut String) {
    use comrak::nodes::NodeValue;
    match &node.data.borrow().value {
        NodeValue::Text(text) => {
            if let Ok(text) = std::str::from_utf8(text) {
                output.push_str(&escape_text(text));
            }
        },
        //NodeValue::Code(text) => {
        //    if let Ok(text) = std::str::from_utf8(text) {
        //        output.push_str("\\texttt{");
        //        output.push_str(&escape_text(text));
        //        output.push_str("}");
        //    }
        //},
        //NodeValue::Emph => {
        //    output.push_str("\\emph{");
        //    for child in node.children() { format_text(child, output); }
        //    output.push_str("}");
        //},
        //NodeValue::Strong => {
        //    output.push_str("\\textbf{");
        //    for child in node.children() { format_text(child, output); }
        //    output.push_str("}");
        //},
        //NodeValue::Strikethrough => {
        //    output.push_str("\\sout{");
        //    for child in node.children() { format_text(child, output); }
        //    output.push_str("}");
        //},
        //NodeValue::Superscript => {
        //    output.push_str("\\textsuperscript{");
        //    for child in node.children() { format_text(child, output); }
        //    output.push_str("}");
        //},
        _ => for child in node.children() { format_text(child, output); },
    }
}

fn format_node<'a>(section_offset: u32, node: &'a comrak::nodes::AstNode<'a>, output: &mut String) {
    use comrak::nodes::NodeValue;
    match &node.data.borrow().value {
        NodeValue::Document => for child in node.children() { format_node(section_offset, child, output); },
        NodeValue::BlockQuote => {
            output.push_str("\\begin{quote}\n");
            for child in node.children() { format_node(section_offset, child, output); }
            output.push_str("\\end{quote}\n");
        },
        NodeValue::List(node_list) => {
            match node_list.list_type {
                comrak::nodes::ListType::Bullet => output.push_str("\\begin{itemize}\n"),
                comrak::nodes::ListType::Ordered => output.push_str("\\begin{enumerate}\n"),
            }
            for child in node.children() { format_node(section_offset, child, output); }
            match node_list.list_type {
                comrak::nodes::ListType::Bullet => output.push_str("\\end{itemize}\n"),
                comrak::nodes::ListType::Ordered => output.push_str("\\end{enumerate}\n"),
            }
        },
        NodeValue::Item(_) => {
            output.push_str("\\item ");
            let mut item: String = String::default();
            for child in node.children() { format_node(section_offset, child, &mut item); }
            output.push_str(item.trim());
            output.push_str("\n");
        },
        NodeValue::DescriptionList => {
            output.push_str("\\begin{description}\n");
            let mut items: String = String::default();
            for child in node.children() { format_node(section_offset, child, &mut items); }
            output.push_str(&items.replace("\n\n\n", "\n").replace("\n\n", "\n"));
            output.push_str("\\end{description}\n");
        },
        NodeValue::DescriptionItem(_) => for child in node.children() { format_node(section_offset, child, output); },
        NodeValue::DescriptionTerm => {
            output.push_str("\\item [");
            let mut term: String = String::default();
            for child in node.children() { format_text(child, &mut term); }
            output.push_str(term.trim());
            output.push_str("] ");
        },
        NodeValue::DescriptionDetails => {
            for child in node.children() { format_node(section_offset, child, output); }
            output.push_str("\n");
        },
        NodeValue::CodeBlock(node_code_block) => {
            let lang = std::str::from_utf8(&node_code_block.info).expect("valid utf-8");
            let source = std::str::from_utf8(&node_code_block.literal).expect("valid utf-8");

            if lang.to_lowercase() == "plantuml" {
                log::debug!("TODO: render plantuml");
                return;
            }
            else if lang.to_lowercase() == "katex" {
                output.push_str("\\begin{equation}\n");
                output.push_str(source);
                output.push_str("\\end{equation}\n");
                return;
            }

            let lang = if lang.trim().is_empty() { "text" } else { lang };
            output.push_str("\\begin{absolutelynopagebreak}\n\\begin{minted}[breaklines]{");
            output.push_str(&escape_text(lang));
            output.push_str("}\n");
            output.push_str(source);
            output.push_str("\\end{minted}\n\\end{absolutelynopagebreak}\n\n");
        },
        NodeValue::HtmlBlock(node_html_block) => {
            log::warn!("can't handle html block, rendering it as syntax...");
            let source = std::str::from_utf8(&node_html_block.literal).expect("valid utf-8");
            output.push_str("\\begin{minted}[breaklines]{text}\n");
            output.push_str(source);
            output.push_str("\\end{minted}\n\n");
        },
        NodeValue::Paragraph => {
            for child in node.children() {
                format_node(section_offset, child, output);
            }
            output.push_str("\n\n");
        },
        NodeValue::Heading(node_heading) => {
            match node_heading.level + section_offset {
                1 => output.push_str("\\section{"),
                2 => output.push_str("\\subsection{"),
                3 => output.push_str("\\subsubsection{"),
                4 => output.push_str("\\paragraph{"),
                5 => output.push_str("\\subparagraph{"),
                _ => output.push_str("\\textbf{"),
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
                output.push_str(&escape_text(text));
            }
        },
        NodeValue::TaskItem(checked) => {
            let mut item: String = String::default();
            for child in node.children() { format_node(section_offset, child, &mut item); }
            if *checked {
                output.push_str(r"$\text{\rlap{$\checkmark$}}\square$ ");
            }
            else {
                output.push_str(r"$\square$ ");
            }
            output.push_str(item.trim());
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
                output.push_str(&escape_text(text));
                output.push_str("|");
            }
        },
        NodeValue::HtmlInline(text) => {
            let source = std::str::from_utf8(&text).expect("valid utf-8");
            log::warn!("can't handle inline html `{}`, rendering it as syntax...", source);
            output.push_str("\\mintinline{html}{");
            output.push_str(&escape_text(source));
            output.push_str("}");
        },
        NodeValue::Emph => {
            output.push_str("\\emph{");
            for child in node.children() { format_node(section_offset, child, output); }
            output.push_str("}");
        },
        NodeValue::Strong => {
            output.push_str("\\textbf{");
            for child in node.children() { format_node(section_offset, child, output); }
            output.push_str("}");
        },
        NodeValue::Strikethrough => {
            output.push_str("\\sout{");
            for child in node.children() { format_node(section_offset, child, output); }
            output.push_str("}");
        },
        NodeValue::Superscript => {
            output.push_str("\\textsuperscript{");
            for child in node.children() { format_node(section_offset, child, output); }
            output.push_str("}");
        },
        NodeValue::Link(node_link) => {
            let url = std::str::from_utf8(&node_link.url).expect("valid utf-8");
            output.push_str("\\href{");
            output.push_str(&escape_text(url));
            output.push_str("}{");
            for child in node.children() { format_text(child, output); }
            output.push_str("}\\footnote{\\url{");
            output.push_str(&escape_text(url));
            output.push_str("}}");
        },
        NodeValue::Image(node_link) => {
            
        },
        NodeValue::FootnoteReference(text) => {
            
        },
    }
}

fn format_markdown(section_offset: u32, src: &str) -> Result<String, Box<dyn std::error::Error>> {
    let arena = comrak::Arena::new();
    let root = comrak::parse_document(
        &arena,
        src,
        &COMRAK_OPTIONS);

    let mut latex: String = String::with_capacity(src.len());
    format_node(section_offset, &root, &mut latex);

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
    book.description = format_markdown(0, &book.description)?;
    for chapter in book.chapters.iter_mut() {
        chapter.contents = format_markdown(0, &chapter.contents)?;
        for section in chapter.sections.iter_mut() {
            section.contents = format_markdown(1, &section.contents)?;
        }
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