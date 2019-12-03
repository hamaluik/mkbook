use std::path::{PathBuf, Path};
use std::{fs, io};
use comrak::ComrakOptions;
use syntect::{parsing::SyntaxSet, highlighting::{ThemeSet, Theme}};
use askama::Template;

use super::models::frontmatter::{ParsedFrontMatter, FrontMatter};
use super::models::chapter::{Chapter};
use super::extensions::{create_plantuml_svg, create_katex_inline};

pub const STYLESHEET: &'static str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));
pub const ASSET_FAVICON: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/favicon.ico"));
pub const ASSET_ICONS: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons.svg"));

pub const SYNTAX_TOML: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/syntaxes/TOML.sublime-syntax"));
pub const SYNTAX_HAXE: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/syntaxes/haxe.sublime-syntax"));
pub const SYNTAX_HXML: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/syntaxes/hxml.sublime-syntax"));
pub const SYNTAX_SASS: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/syntaxes/Sass.sublime-syntax"));
pub const SYNTAX_SCSS: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/syntaxes/SCSS.sublime-syntax"));

lazy_static! {
    static ref HIGHLIGHT_SYNTAX_SETS: SyntaxSet = {
        use syntect::parsing::SyntaxDefinition;

        let ss = SyntaxSet::load_defaults_newlines();
        let mut ssb = ss.into_builder();
        ssb.add(SyntaxDefinition::load_from_str(SYNTAX_TOML, true, None).expect("valid TOML syntax definition"));
        ssb.add(SyntaxDefinition::load_from_str(SYNTAX_HAXE, true, None).expect("valid haxe syntax definition"));
        ssb.add(SyntaxDefinition::load_from_str(SYNTAX_HXML, true, None).expect("valid hxml syntax definition"));
        ssb.add(SyntaxDefinition::load_from_str(SYNTAX_SASS, true, None).expect("valid sass syntax definition"));
        ssb.add(SyntaxDefinition::load_from_str(SYNTAX_SCSS, true, None).expect("valid scss syntax definition"));
        let ss = ssb.build();
    
        //if cfg!(debug_assertions) {
        //    let mut syntaxes: Vec<(String, String)> = ss.syntaxes().iter()
        //        .map(|s| (s.name.clone(), s.file_extensions.iter().map(|s| &**s).collect::<Vec<&str>>().join("`, `")))
        //        .collect();
        //    syntaxes.sort_by(|a, b| a.0.cmp(&b.0));
        //    for syntax in syntaxes {
        //        println!("{}\n\n: `{}`\n\n", syntax.0, syntax.1);
        //    }
        //}

        ss
    };
    static ref HIGHLIGHT_THEME_SETS: ThemeSet = ThemeSet::load_defaults();
    static ref HIGHLIGHT_THEME: &'static Theme = &HIGHLIGHT_THEME_SETS.themes["base16-eighties.dark"];
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

mod filters;

struct FormatResponse {
    output: String,
    include_katex_css: bool,
}

fn format_code(lang: &str, src: &str) -> Result<FormatResponse, Box<dyn std::error::Error>> {
    use syntect::parsing::SyntaxReference;
    use syntect::html::highlighted_html_for_string;

    // render plantuml code blocks into an inline svg
    if lang == "plantuml" {
        return Ok(FormatResponse {
            output: create_plantuml_svg(src)?,
            include_katex_css: false,
        });
    }
    // render katex code blocks into an inline math
    if lang == "katex" {
        return Ok(FormatResponse {
            output: create_katex_inline(src)?,
            include_katex_css: true,
        });
    }

    let syntax: Option<&SyntaxReference> = if lang.len() > 0 {
        let syntax = HIGHLIGHT_SYNTAX_SETS.find_syntax_by_token(lang);
        if syntax.is_none() {
            eprintln!("warning: language `{}` not recognized, formatting code block as plain text!", lang);
        }
        syntax
    }
    else {
        None
    };
    let syntax = syntax.unwrap_or(HIGHLIGHT_SYNTAX_SETS.find_syntax_plain_text());

    let html = highlighted_html_for_string(src, &HIGHLIGHT_SYNTAX_SETS, &syntax, &HIGHLIGHT_THEME);

    Ok(FormatResponse {
        output: html,
        include_katex_css: false,
    })
}

fn wrap_image_in_figure(link: &comrak::nodes::NodeLink, alt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let title = String::from_utf8_lossy(link.title.as_ref());
    let url = String::from_utf8_lossy(link.url.as_ref());
    if title.len() > 0 {
        Ok(format!(r#"<figure><img src="{}" alt="{}" title="{}"><figcaption>{}</figcaption></figure>"#, url, alt, title, title))
    }
    else {
        Ok(format!(r#"<figure><img src="{}" alt="{}"></figure>"#, url, alt))
    }
}

fn format_markdown(src: &str) -> Result<FormatResponse, Box<dyn std::error::Error>> {
    use comrak::{Arena, parse_document, format_html};
    use comrak::nodes::{AstNode, NodeValue};

    let arena = Arena::new();

    let root = parse_document(
        &arena,
        src,
        &COMRAK_OPTIONS);

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F) -> Result<(), Box<dyn std::error::Error>>
        where F : FnMut(&'a AstNode<'a>) -> Result<(), Box<dyn std::error::Error>> {
        f(node)?;
        for c in node.children() {
            iter_nodes(c, f)?;
        }
        Ok(())
    }

    let mut use_katex_css = false;
    iter_nodes(root, &mut |node| {
        let value = &mut node.data.borrow_mut().value;
        match value {
            NodeValue::CodeBlock(ref block) => {
                let lang = String::from_utf8_lossy(block.info.as_ref());
                let source = String::from_utf8_lossy(block.literal.as_ref());
                let FormatResponse { output, include_katex_css } = format_code(&lang, &source)?;
                if include_katex_css {
                    use_katex_css = true;
                }
                let highlighted: Vec<u8> = Vec::from(output.into_bytes());
                *value = NodeValue::HtmlInline(highlighted);
            },
            NodeValue::Paragraph => {
                if node.children().count() == 1 {
                    let first_child = &node.first_child().unwrap();
                    let first_value = &first_child.data.borrow().value;
                    if let NodeValue::Image(link) = first_value {
                        if first_child.children().count() > 0 {
                            let mut alt: String = String::default();
                            for child in first_child.children() {
                                if let NodeValue::Text(t) = &child.data.borrow().value {
                                    alt.push_str(&String::from_utf8_lossy(&t));
                                }
                                child.detach();
                            }
                            first_child.detach();
                            let figure = wrap_image_in_figure(&link, &alt)?;
                            let figure: Vec<u8> = Vec::from(figure.into_bytes());
                            *value = NodeValue::HtmlInline(figure);
                        }
                    }
                }
            },
            _ => {}
        }
        Ok(())
    })?;

    let mut output: Vec<u8> = Vec::with_capacity((src.len() as f64 * 1.2) as usize);
    format_html(root, &COMRAK_OPTIONS, &mut output).expect("can format HTML");
    let output = String::from_utf8(output).expect("valid utf-8 generated HTML");
    Ok(FormatResponse {
        output,
        include_katex_css: use_katex_css,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_figures() {
        let src = r#"![bear](https://placebear.com/g/512/256 "A majestic bear")"#;
        let result = format_markdown(src).expect("can format");
        assert_eq!(result.output, r#"<figure><img src="https://placebear.com/g/512/256" alt="bear" title="A majestic bear"><figcaption>A majestic bear</figcaption></figure>"#);
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a, 'b, 'c> {
    book: &'a FrontMatter,
    chapters: &'b Vec<Chapter>,
    book_description: &'c str,
    include_katex_css: bool,
    include_reload_script: bool,
}

fn generate_index<W: io::Write>(book: &FrontMatter, content: String, include_katex_css: bool, chapters: &Vec<Chapter>, mut output: W, include_reload_script: bool) -> Result<(), Box<dyn std::error::Error>> {
    // fill out our template
    let template = IndexTemplate {
        book,
        chapters,
        book_description: &content,
        include_katex_css,
        include_reload_script,
    };

    // and render!
    let s = template.render()?;
    output.write_all(s.as_bytes())?;

    Ok(())
}

#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate<'a, 'b, 'c, 'd, 'e, 'g> {
    chapter: &'a Chapter,
    content: &'b str,
    chapters: &'c Vec<Chapter>,
    prev_chapter: Option<&'d Chapter>,
    next_chapter: Option<&'e Chapter>,
    book: &'g FrontMatter,
    include_katex_css: bool,
    include_reload_script: bool,
}

fn format_page<W: io::Write>(book: &FrontMatter, chapter: &Chapter, chapters: &Vec<Chapter>, prev_chapter: Option<&Chapter>, next_chapter: Option<&Chapter>, content: &str, include_katex_css: bool, mut output: W, include_reload_script: bool) -> Result<(), Box<dyn std::error::Error>> {
    // fill out our template
    let template = PageTemplate {
        chapter,
        content,
        chapters,
        prev_chapter,
        next_chapter,
        book,
        include_katex_css,
        include_reload_script,
    };

    // and render!
    let s = template.render()?;
    output.write_all(s.as_bytes())?;

    Ok(())
}

pub fn build<PIn: AsRef<Path>, POut: AsRef<Path>>(src: PIn, dest: POut, include_reload_script: bool) -> Result<(), Box<dyn std::error::Error>> {
    let src = PathBuf::from(src.as_ref());
    let dest = PathBuf::from(dest.as_ref());
    if !dest.exists() {
        std::fs::create_dir_all(&dest)?;
        log::info!("created directory `{}`...", dest.display());
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
    let FormatResponse { output, include_katex_css } = format_markdown(&book_description)?;
    let book_description = output;

    // load all our chapters
    let mut chapters: Vec<Chapter> = Vec::default();
    for entry in src.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            // try to find a `README.md` file and parse it to get the chapter's title, fall back to the directory
            // name if we can't do that
            let chapter_name = path.file_name().map(std::ffi::OsStr::to_str).flatten().unwrap_or_default();
            let index_path = path.join("README.md");
            let (front, contents) = if index_path.exists() {
                let contents = fs::read_to_string(&index_path)?;
                let (front, contents) = super::extract_frontmatter(&contents)?;
                let front = front.unwrap_or_default().into_front(&book_front, chapter_name, &format!("{}/index.html", chapter_name));
                (front, contents)
            }
            else {
                (ParsedFrontMatter::default().into_front(&book_front, chapter_name, &format!("{}/index.html", chapter_name)), String::new())
            };

            let mut chapter: Chapter = Chapter {
                front,
                sections: Vec::default(),
                source: path.clone(),
                contents,
            };

            for entry in path.read_dir()? {
                let entry = entry?;
                let path = entry.path();
                if let Some("md") = path.extension().map(std::ffi::OsStr::to_str).flatten() {
                    let name = path.file_stem().map(std::ffi::OsStr::to_str).flatten();
                    if name.is_none() { continue; }
                    let name = name.unwrap();
                    if name == "README" {
                        continue;
                    }
    
                    let contents = fs::read_to_string(&path)?;
                    let (front, contents) = super::extract_frontmatter(&contents)?;
                    let front = front.unwrap_or_default().into_front(&book_front, name, &format!("{}/{}.html", chapter_name, name));
                    chapter.sections.push(Chapter {
                        front,
                        sections: Vec::new(),
                        source: path,
                        contents,
                    });
                }
            }

            chapters.push(chapter);
        }
        else if let Some("md") = path.extension().map(std::ffi::OsStr::to_str).flatten() {
            let name = path.file_stem().map(std::ffi::OsStr::to_str).flatten();
            if name.is_none() { continue; }
            let name = name.unwrap();
            if name == "README" {
                continue;
            }

            let contents = fs::read_to_string(&path)?;
            let (front, contents) = super::extract_frontmatter(&contents)?;
            let front = front.unwrap_or_default().into_front(&book_front, name, &format!("{}/index.html", name));
            chapters.push(Chapter {
                front,
                sections: Vec::new(),
                source: path,
                contents,
            });
        }
    }

    // sort all the chapters
    chapters.sort_by(|a, b| a.front.url.cmp(&b.front.url));
    for chapter in chapters.iter_mut() {
        chapter.sections.sort_by(|a, b| a.front.url.cmp(&b.front.url));
    }

    // generate our index
    let index_out_path = dest.join("index.html");
    let index_out = fs::File::create(&index_out_path)?;
    let index_out = io::BufWriter::new(index_out);
    generate_index(&book_front, book_description, include_katex_css, &chapters, index_out, include_reload_script)?;
    log::info!("Rendered index into `{}`", index_out_path.display());

    // compile markdown and write the actual pages
    let mut prev_chapter = None;
    for (chapter_index, chapter) in chapters.iter().enumerate() {
        // render the index
        let chapter_root = dest.join(chapter.source.file_stem().map(std::ffi::OsStr::to_str).flatten().unwrap());
        let out = chapter_root.join("index.html");
        log::info!("Rendering `{}` into `{}`...", chapter.source.display(), out.display());
        fs::create_dir_all(&chapter_root)?;

        let outfile = fs::File::create(&out)?;
        let outfile = io::BufWriter::new(outfile);

        let FormatResponse { output, include_katex_css } = format_markdown(&chapter.contents)?;

        let next_chapter = 
            if chapter.sections.len() > 0 {
                Some(chapter.sections.iter().nth(0).expect("section 0 exists"))
            }
            else if chapter_index < chapters.len() - 1 {
                Some(chapters.iter().nth(chapter_index + 1).expect("chapter n+1 exists"))
            }
            else {
                None
            };

        format_page(&book_front, &chapter, &chapters, prev_chapter, next_chapter, &output, include_katex_css, outfile, include_reload_script)?;
        prev_chapter = Some(chapter);

        // now the sections
        for (section_index, section) in chapter.sections.iter().enumerate() {
            let name = section.source.file_stem().map(std::ffi::OsStr::to_str).flatten().unwrap();
            let out = chapter_root.join(&format!("{}.html", name));
            log::info!("Rendering `{}` into `{}`...", section.source.display(), out.display());

            let outfile = fs::File::create(&out)?;
            let outfile = io::BufWriter::new(outfile);

            let FormatResponse { output, include_katex_css } = format_markdown(&section.contents)?;

            let next_chapter = if section_index < chapter.sections.len() - 1 {
                Some(chapter.sections.iter().nth(section_index + 1).expect("chapter n+1 exists"))
            }
            else if chapter_index < chapters.len() - 1 {
                Some(chapters.iter().nth(chapter_index + 1).expect("chapter n+1 exists"))
            }
            else {
                None
            };

            format_page(&book_front, &section, &chapters, prev_chapter, next_chapter, &output, include_katex_css, outfile, include_reload_script)?;
            prev_chapter = Some(section);

        }
    }

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
                    let dest_path: PathBuf = dest.join(entry.path().iter().skip(1).map(PathBuf::from).collect::<PathBuf>());
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

    // save the built-in assets
    fs::write(dest.join("style.css"), STYLESHEET)?;
    log::info!("Wrote {}", dest.join("style.css").display());
    fs::write(dest.join("favicon.ico"), ASSET_FAVICON)?;
    log::info!("Wrote {}", dest.join("favicon.ico").display());
    fs::write(dest.join("icons.svg"), ASSET_ICONS)?;
    log::info!("Wrote {}", dest.join("icons.svg").display());

    log::info!("Done!");
    Ok(())
}