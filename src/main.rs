#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use std::{fs, io};
use comrak::ComrakOptions;
use syntect::{parsing::SyntaxSet, highlighting::{ThemeSet, Theme}};
use askama::Template;

pub const STYLESHEET: &'static str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));
pub const ASSET_FAVICON: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/favicon.ico"));
pub const ASSET_ICONS: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons.svg"));
pub const ASSET_DEFAULT_MKBOOK: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/mkbook.default.toml"));
pub const ASSET_DEFAULT_INTRODUCTION: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/01-introduction.default.md"));

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

mod cli;
mod models;
mod filters;

use models::frontmatter::{ParsedFrontMatter, FrontMatter};
use models::book::{ParsedBook, Book};
use models::chapter::{Chapter};

fn format_code(lang: &str, src: &str) -> Result<String, Box<dyn std::error::Error>> {
    use syntect::parsing::SyntaxReference;
    use syntect::html::highlighted_html_for_string;

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

    Ok(html)
}

fn extract_frontmatter(src: &str) -> Result<(Option<ParsedFrontMatter>, String), Box<dyn std::error::Error>> {
    if src.starts_with("---\n") {
        let slice = &src[4..];
        let end = slice.find("---\n");
        if end.is_none() {
            return Ok((None, src.to_owned()));
        }
        let end = end.unwrap();

        let front = &slice[..end];
        let contents = &slice[end+4..];
        let front: ParsedFrontMatter = toml::from_str(front)?;
        Ok((Some(front), contents.to_owned()))
    }
    else if src.starts_with("---\r\n") {
        let slice = &src[5..];
        let end = slice.find("---\r\n");
        if end.is_none() {
            return Ok((None, src.to_owned()));
        }
        let end = end.unwrap();

        let front = &slice[..end];
        let contents = &slice[end+5..];
        let front: ParsedFrontMatter = toml::from_str(front)?;
        Ok((Some(front), contents.to_owned()))
    }
    else {
        Ok((None, src.to_owned()))
    }
}

fn format_markdown(src: &str) -> Result<String, Box<dyn std::error::Error>> {
    use comrak::{Arena, parse_document, format_html};
    use comrak::nodes::{AstNode, NodeValue};

    let arena = Arena::new();

    let root = parse_document(
        &arena,
        src,
        &COMRAK_OPTIONS);

    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F) -> Result<(), Box<dyn std::error::Error>>
        where F : Fn(&'a AstNode<'a>) -> Result<(), Box<dyn std::error::Error>> {
        f(node)?;
        for c in node.children() {
            iter_nodes(c, f)?;
        }
        Ok(())
    }

    iter_nodes(root, &|node| {
        let value = &mut node.data.borrow_mut().value;
        if let NodeValue::CodeBlock(ref block) = value {
            let lang = String::from_utf8(block.info.clone()).expect("code lang is utf-8");
            let source = String::from_utf8(block.literal.clone()).expect("source code is utf-8");
            let highlighted: String = format_code(&lang, &source)?;
            let highlighted: Vec<u8> = Vec::from(highlighted.into_bytes());

            *value = NodeValue::HtmlBlock(comrak::nodes::NodeHtmlBlock {
                literal: highlighted,
                block_type: 0,
            });
        }
        Ok(())
    })?;

    let mut output: Vec<u8> = Vec::with_capacity((src.len() as f64 * 1.2) as usize);
    format_html(root, &COMRAK_OPTIONS, &mut output).expect("can format HTML");
    let output = String::from_utf8(output).expect("valid utf-8 generated HTML");
    Ok(output)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a, 'b> {
    book: &'a Book,
    chapters: &'b Vec<Chapter>,
}

fn generate_index<W: io::Write>(book: &Book, chapters: &Vec<Chapter>, mut output: W) -> Result<(), Box<dyn std::error::Error>> {
    // fill out our template
    let template = IndexTemplate {
        book,
        chapters,
    };

    // and render!
    let s = template.render()?;
    output.write_all(s.as_bytes())?;

    Ok(())
}

#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    chapter: &'a Chapter,
    content: &'b str,
    url: &'f str,
    chapters: &'c Vec<Chapter>,
    prev_chapter: Option<&'d Chapter>,
    next_chapter: Option<&'e Chapter>,
    book: &'g Book,
}

fn format_page<W: io::Write>(book: &Book, chapter: &Chapter, chapters: &Vec<Chapter>, prev_chapter: Option<&Chapter>, next_chapter: Option<&Chapter>, url: &str, content: &str, mut output: W) -> Result<(), Box<dyn std::error::Error>> {
    // fill out our template
    let template = PageTemplate {
        chapter,
        content,
        url,
        chapters,
        prev_chapter,
        next_chapter,
        book,
    };

    // and render!
    let s = template.render()?;
    output.write_all(s.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::build_cli().get_matches();

    if let Some(submatches) = matches.subcommand_matches("init") {
        let dest = submatches.value_of("directory").expect("directory value");
        let dest = PathBuf::from(dest);

        println!("Initializing a book into {}...", dest.display());
        fs::create_dir_all(&dest)?;
        let book_toml_path = dest.join("mkbook.toml");
        fs::write(&book_toml_path, ASSET_DEFAULT_MKBOOK)?;
        let intro_path = dest.join("01-introduction.md");
        fs::write(&intro_path, ASSET_DEFAULT_INTRODUCTION)?;
        println!("Done!");

        println!("You can now build your book by running:");
        if dest.display().to_string() != "src" {
            println!("mkbook build -i {}", dest.display());
        }
        else {
            println!("mkbook build");
        }

        Ok(())
    }
    else if let Some(submatches) = matches.subcommand_matches("build") {
        let src = submatches.value_of("in").expect("in value");
        let dest = submatches.value_of("out").expect("out value");

        let src = PathBuf::from(src);
        let dest = PathBuf::from(dest);
        std::fs::create_dir_all(&dest)?;

        // load our book
        let book_toml_path = src.join("mkbook.toml");
        let parsed_book: Option<ParsedBook> = if book_toml_path.exists() {
            let contents = fs::read_to_string(&book_toml_path)?;
            let contents: ParsedBook = toml::from_str(&contents)?;
            Some(contents)
        }
        else {
            None
        };
        let parsed_book = parsed_book.unwrap_or_default();
        let book: Book = parsed_book.into();

        // load all our chapters
        let mut chapters: Vec<Chapter> = Vec::default();
        for entry in src.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                // try to find a `chapter.toml` file and parse it to get the chapter's title, fall back to the directory
                // name if we can't do that
                let chapter_name = path.file_name().map(std::ffi::OsStr::to_str).flatten().unwrap_or_default();
                let index_path = path.join("index.md");
                let (front, contents) = if index_path.exists() {
                    let contents = fs::read_to_string(&index_path)?;
                    let (front, contents) = extract_frontmatter(&contents)?;
                    let front = front.unwrap_or_default().into_front(chapter_name);
                    (front, contents)
                }
                else {
                    (FrontMatter {
                        title: chapter_name.to_owned(),
                    }, String::new())
                };

                let mut chapter: Chapter = Chapter {
                    url: format!("{}/index.html", chapter_name),
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
                        if name == "index" {
                            continue;
                        }
        
                        let contents = fs::read_to_string(&path)?;
                        let (front, contents) = extract_frontmatter(&contents)?;
                        let front = front.unwrap_or_default().into_front(name);
                        chapter.sections.push(Chapter {
                            url: format!("{}/{}.html", chapter_name, name),
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

                let contents = fs::read_to_string(&path)?;
                let (front, contents) = extract_frontmatter(&contents)?;
                let front = front.unwrap_or_default().into_front(name);
                chapters.push(Chapter {
                    url: format!("{}/index.html", name),
                    front,
                    sections: Vec::new(),
                    source: path,
                    contents,
                });
            }
        }

        // sort all the chapters
        chapters.sort_by(|a, b| a.url.cmp(&b.url));
        for chapter in chapters.iter_mut() {
            chapter.sections.sort_by(|a, b| a.url.cmp(&b.url));
        }

        // generate our index
        let index_out_path = dest.join("index.html");
        let index_out = fs::File::create(&index_out_path)?;
        let index_out = io::BufWriter::new(index_out);
        generate_index(&book, &chapters, index_out)?;
        println!("Rendered index into `{}`", index_out_path.display());

        // compile markdown and write the actual pages
        let mut prev_chapter = None;
        for (chapter_index, chapter) in chapters.iter().enumerate() {
            // render the index
            let chapter_root = dest.join(chapter.source.file_stem().map(std::ffi::OsStr::to_str).flatten().unwrap());
            let out = chapter_root.join("index.html");
            print!("Rendering `{}` into `{}`...", chapter.source.display(), out.display());
            fs::create_dir_all(&chapter_root)?;

            let outfile = fs::File::create(&out)?;
            let outfile = io::BufWriter::new(outfile);

            let contents = format_markdown(&chapter.contents)?;

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

            format_page(&book, &chapter, &chapters, prev_chapter, next_chapter, &chapter.url, &contents, outfile)?;
            prev_chapter = Some(chapter);

            println!(" done!");

            // now the sections
            for (section_index, section) in chapter.sections.iter().enumerate() {
                let name = section.source.file_stem().map(std::ffi::OsStr::to_str).flatten().unwrap();
                let out = chapter_root.join(&format!("{}.html", name));
                print!("Rendering `{}` into `{}`...", section.source.display(), out.display());
    
                let outfile = fs::File::create(&out)?;
                let outfile = io::BufWriter::new(outfile);
    
                let contents = format_markdown(&section.contents)?;

                let next_chapter = if section_index < chapter.sections.len() - 1 {
                    Some(chapter.sections.iter().nth(section_index + 1).expect("chapter n+1 exists"))
                }
                else if chapter_index < chapters.len() - 1 {
                    Some(chapters.iter().nth(chapter_index + 1).expect("chapter n+1 exists"))
                }
                else {
                    None
                };

                format_page(&book, &section, &chapters, prev_chapter, next_chapter, &section.url, &contents, outfile)?;
                prev_chapter = Some(section);
    
                println!(" done!");
            }
        }

        // save the assets
        fs::write(dest.join("style.css"), STYLESHEET)?;
        println!("Wrote {}", dest.join("style.css").display());
        fs::write(dest.join("favicon.ico"), ASSET_FAVICON)?;
        println!("Wrote {}", dest.join("favicon.ico").display());
        fs::write(dest.join("icons.svg"), ASSET_ICONS)?;
        println!("Wrote {}", dest.join("icons.svg").display());

        println!("Done!");
        Ok(())
    }
    else {
        cli::build_cli().print_long_help()?;
        Ok(())
    }
}
