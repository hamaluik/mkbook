use std::path::{Path, PathBuf};
use askama::Template;
use std::{fs, io};
use comrak::ComrakOptions;
use crate::extensions::create_plantuml_svg;

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
        NodeValue::Emph => {
            output.push_str("\\emph{");
            for child in node.children() { format_text(child, output); }
            output.push_str("}");
        },
        NodeValue::Strong => {
            output.push_str("\\textbf{");
            for child in node.children() { format_text(child, output); }
            output.push_str("}");
        },
        NodeValue::Strikethrough => {
            output.push_str("\\sout{");
            for child in node.children() { format_text(child, output); }
            output.push_str("}");
        },
        NodeValue::Superscript => {
            output.push_str("\\textsuperscript{");
            for child in node.children() { format_text(child, output); }
            output.push_str("}");
        },
        _ => for child in node.children() { format_text(child, output); },
    }
}

fn format_node<'a, P: AsRef<Path>>(section_offset: u32, dest_path: P, node: &'a comrak::nodes::AstNode<'a>, output: &mut String) {
    use comrak::nodes::NodeValue;
    match &node.data.borrow().value {
        NodeValue::Document => for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, output); },
        NodeValue::BlockQuote => {
            output.push_str("\\begin{quote}\n");
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, output); }
            output.push_str("\\end{quote}\n");
        },
        NodeValue::List(node_list) => {
            match node_list.list_type {
                comrak::nodes::ListType::Bullet => output.push_str("\\begin{itemize}\n"),
                comrak::nodes::ListType::Ordered => output.push_str("\\begin{enumerate}\n"),
            }
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, output); }
            match node_list.list_type {
                comrak::nodes::ListType::Bullet => output.push_str("\\end{itemize}\n"),
                comrak::nodes::ListType::Ordered => output.push_str("\\end{enumerate}\n"),
            }
        },
        NodeValue::Item(_) => {
            output.push_str("\\item ");
            let mut item: String = String::default();
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, &mut item); }
            output.push_str(item.trim());
            output.push_str("\n");
        },
        NodeValue::DescriptionList => {
            output.push_str("\\begin{description}\n");
            let mut items: String = String::default();
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, &mut items); }
            output.push_str(&items.replace("\n\n\n", "\n").replace("\n\n", "\n"));
            output.push_str("\\end{description}\n");
        },
        NodeValue::DescriptionItem(_) => for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, output); },
        NodeValue::DescriptionTerm => {
            output.push_str("\\item [");
            let mut term: String = String::default();
            for child in node.children() { format_text(child, &mut term); }
            output.push_str(term.trim());
            output.push_str("] ");
        },
        NodeValue::DescriptionDetails => {
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, output); }
            output.push_str("\n");
        },
        NodeValue::CodeBlock(node_code_block) => {
            let lang = std::str::from_utf8(&node_code_block.info).expect("valid utf-8");
            let source = std::str::from_utf8(&node_code_block.literal).expect("valid utf-8");

            if lang.to_lowercase() == "plantuml" {
                let digest = md5::compute(source.as_bytes());
                let output_path = dest_path.as_ref().join(format!("{:x?}.svg", digest));
                let svg = match create_plantuml_svg(&source) {
                    Ok(svg) => svg,
                    Err(e) => {
                        log::error!("failed to create SVG: {:?}", e);
                        return;
                    }
                };
                if let Err(e) = std::fs::write(&output_path, svg) {
                    log::error!("failed to write SVG to file `{}`: {:?}", output_path.display(), e);
                    return;
                }

                output.push_str("\\begin{figure}[H]\n");
                output.push_str("\\centering\n");
                output.push_str("\\includesvg{");
                output.push_str(&format!("{:x?}.svg", digest));
                output.push_str("}\n");
                output.push_str("\\end{figure}\n");

                return;
            }
            else if lang.to_lowercase() == "katex" {
                output.push_str("\\begin{equation}\n");
                output.push_str(source);
                output.push_str("\\end{equation}\n");
                return;
            }
            let lang = if lang.trim().is_empty() { "text" } else { lang };
            output.push_str("\\begin{absolutelynopagebreak}\n\\begin{minted}[breaklines,baselinestretch=1.2,bgcolor=light-grey,fontsize=\\footnotesize,]{");
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
                format_node(section_offset, dest_path.as_ref(), child, output);
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
            output.push_str("}\n\n");
        },
        NodeValue::ThematicBreak => {
            output.push_str("\\hline\n");
        },
        NodeValue::FootnoteDefinition(label) => {
            let label = std::str::from_utf8(&label).expect("valid utf-8");
            log::debug!("footnote definition: {}", label);
            output.push_str("\\footnotetext[");
            output.push_str(&escape_text(label));
            output.push_str("]{");
            let mut definition: String = String::default();
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, &mut definition); }
            output.push_str(definition.trim());
            output.push_str("}\n");
        },
        NodeValue::Table(table_alignments) => {
            use comrak::nodes::TableAlignment;
            let spec: String = table_alignments.iter().map(|a| match a {
                TableAlignment::None => "l",
                TableAlignment::Center => "c",
                TableAlignment::Left => "l",
                TableAlignment::Right => "r",
            }).collect::<Vec<&str>>().join(" ");

            output.push_str("\\begin{center}\n\\begin{tabular}{");
            output.push_str(&spec);
            output.push_str("}\n");

            let mut rows: String = String::default();
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, &mut rows); }
            output.push_str(rows.trim());
            output.push_str("\\end{tabular}\n\\end{center}\n\n");
        },
        NodeValue::TableRow(header) => {
            let row: String = node.children().map(|child| {
                let mut column: String = String::default();
                format_node(section_offset, dest_path.as_ref(), child, &mut column);
                column
            })
            .collect::<Vec<String>>()
            .join(" & ");
            output.push_str(row.trim());
            output.push_str(r" \\");
            if *header {
                output.push_str("\n\\hline\n");
            }
            else {
                output.push_str("\n");
            }
        },
        NodeValue::TableCell => for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, output); },
        NodeValue::Text(text) => {
            if let Ok(text) = std::str::from_utf8(text) {
                output.push_str(&escape_text(text));
            }
        },
        NodeValue::TaskItem(checked) => {
            let mut item: String = String::default();
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, &mut item); }
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
                //output.push_str(&escape_text(text));
                output.push_str(text);
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
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, output); }
            output.push_str("}");
        },
        NodeValue::Strong => {
            output.push_str("\\textbf{");
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, output); }
            output.push_str("}");
        },
        NodeValue::Strikethrough => {
            output.push_str("\\sout{");
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, output); }
            output.push_str("}");
        },
        NodeValue::Superscript => {
            output.push_str("\\textsuperscript{");
            for child in node.children() { format_node(section_offset, dest_path.as_ref(), child, output); }
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
            let url = std::str::from_utf8(&node_link.url).expect("valid utf-8");
            let title = std::str::from_utf8(&node_link.title).expect("valid utf-8");
            if url.starts_with("http://") || url.starts_with("https://") {
                // TODO: download images?
                //log::warn!("skipping image `{}` as we can't download images yet!", url);
                let client = reqwest::Client::new();
                let mut res = match client.get(url).send() {
                    Ok(r) => r,
                    Err(e) => {
                        log::error!("failed to download image `{}`, reqwest error: {:?}", url, e);
                        return;
                    }
                };
                if !res.status().is_success() {
                    log::warn!("failed to download image at url `{}`: error {}: {:?}", url, res.status().as_u16(), res.text());
                    return;
                }
                
                let image_type = match res.headers().get("Content-Type").map(reqwest::header::HeaderValue::to_str) {
                    Some(Ok("image/jpeg")) => Some("jpg"),
                    Some(Ok("image/png")) => Some("png"),
                    Some(Ok("application/pdf")) => Some("pdf"),
                    Some(Ok("application/postscript")) => Some("eps"),
                    _ => None,
                };
                let image_type = match image_type {
                    Some(t) => t,
                    None => {
                        log::warn!("unhandled content-type for url `{}`: {:?}, skipping", url, res.headers().get("Content-Type"));
                        return;
                    }
                };

                // get the md5 of the URL to save as the filename
                let filename = md5::compute(url.as_bytes());
                let filename = format!("{:x?}.{}", filename, image_type);
                let dest = PathBuf::from(dest_path.as_ref()).join(&filename);
                log::info!("saving `{}` to `{}`...", url, dest.display());
                
                let f = match fs::File::create(&dest) {
                    Ok(f) => f,
                    Err(e) => {
                        log::error!("failed to create file `{}`: {:?}", dest.display(), e);
                        return;
                    }
                };
                let mut wtr = io::BufWriter::new(f);
                if let Err(e) = res.copy_to(&mut wtr) {
                    log::error!("failed to download `{}` into `{}`: {:?}", url, dest.display(), e);
                    return;
                }

                // and finally, now that we've downloaded the image, emit our code
                output.push_str("\\begin{figure}[H]\n");
                output.push_str("\\centering\n");
                output.push_str("\\includegraphics[width=\\maxwidth{\\textwidth}]{");
                output.push_str(&filename);
                output.push_str("}\n");
                output.push_str("\\caption{");
                output.push_str(title);
                output.push_str("}\n");
                output.push_str("\\end{figure}\n");
            }
            else {
                // TODO: make sure the file exists?
                output.push_str("\\begin{figure}[h]\n");
                output.push_str("\\centering\n");
                output.push_str("\\includegraphics[width=\\maxwidth{\\textwidth}]{");
                output.push_str(url);
                output.push_str("}\n");
                output.push_str("\\caption{");
                output.push_str(title);
                output.push_str("}\n");
                output.push_str("\\end{figure}\n");
            }
        },
        NodeValue::FootnoteReference(label) => {
            let label = std::str::from_utf8(&label).expect("valid utf-8");
            log::debug!("footnote reference: {}", label);
            output.push_str("\\footnotemark[");
            output.push_str(&escape_text(label));
            output.push_str("]");
        },
    }
}

fn format_markdown<P: AsRef<Path>>(section_offset: u32, dest_path: P, src: &str) -> Result<String, Box<dyn std::error::Error>> {
    let arena = comrak::Arena::new();
    let root = comrak::parse_document(
        &arena,
        src,
        &COMRAK_OPTIONS);

    let mut latex: String = String::with_capacity(src.len());
    format_node(section_offset, dest_path, &root, &mut latex);

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
    let dest_path = dest.parent().unwrap_or(Path::new(".")).to_owned();

    // load the book
    let mut book = super::load_book(&src)?;

    // then convert all the markdown
    book.description = format_markdown(0, &dest_path, &book.description)?;
    for chapter in book.chapters.iter_mut() {
        chapter.contents = format_markdown(0, &dest_path, &chapter.contents)?;
        for section in chapter.sections.iter_mut() {
            section.contents = format_markdown(1, &dest_path, &section.contents)?;
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

    // copy the assets
    for entry in ignore::Walk::new(&src) {
        let entry = entry?;
        if let Some(t) = entry.file_type() {
            if t.is_file() {
                if let Some("md") = entry.path().extension().map(std::ffi::OsStr::to_str).flatten() {
                    // ignore markdown files
                }
                else {
                    // we found an asset to copy!
                    let dest_path: PathBuf = dest_path.join(entry.path().iter().skip(1).map(PathBuf::from).collect::<PathBuf>());
                    if let Some(parent) = dest_path.parent() {
                        if !parent.exists() {
                            fs::create_dir_all(parent)?;
                            log::info!("created directory `{}`...", parent.display());
                        }
                    }
                    fs::copy(entry.path(), &dest_path)?;
                    log::info!("Copied `{}` to `{}`...", entry.path().display(), dest_path.display());
                }
            }
        }
    }

    Ok(())
}