use std::path::PathBuf;
use std::{fs, io};

pub const STYLESHEET: &'static str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));
pub const ASSET_FAVICON: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/favicon.ico"));
pub const ASSET_ICONS: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons.svg"));

mod cli;
mod models;

fn format_code(lang: &str, src: &str) -> Result<String, Box<dyn std::error::Error>> {
    use syntect::parsing::{SyntaxSet, SyntaxReference};
    use syntect::highlighting::{ThemeSet};
    use syntect::html::highlighted_html_for_string;

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-eighties.dark"];

    let syntax: Option<&SyntaxReference> = if lang.len() > 0 {
        let syntax = ss.find_syntax_by_token(lang);
        if syntax.is_none() {
            eprintln!("warning: language `{}` not recognized, formatting code block as plain text!", lang);
        }
        syntax
    }
    else {
        None
    };
    let syntax = syntax.unwrap_or(ss.find_syntax_plain_text());

    let html = highlighted_html_for_string(src, &ss, &syntax, &theme);

    Ok(html)
}

fn extract_frontmatter(src: &str) -> Result<(Option<models::frontmatter::ParsedFrontMatter>, String), Box<dyn std::error::Error>> {
    if src.starts_with("---\n") {
        let slice = &src[4..];
        let end = slice.find("---\n");
        if end.is_none() {
            return Ok((None, src.to_owned()));
        }
        let end = end.unwrap();

        let front = &slice[..end];
        let contents = &slice[end+4..];
        let front: models::frontmatter::ParsedFrontMatter = toml::from_str(front)?;
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
        let front: models::frontmatter::ParsedFrontMatter = toml::from_str(front)?;
        Ok((Some(front), contents.to_owned()))
    }
    else {
        Ok((None, src.to_owned()))
    }
}

fn format_markdown(src: &str) -> Result<String, Box<dyn std::error::Error>> {
    use comrak::{Arena, parse_document, format_html, ComrakOptions};
    use comrak::nodes::{AstNode, NodeValue};

    let options: ComrakOptions = ComrakOptions {
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

    let arena = Arena::new();

    let root = parse_document(
        &arena,
        src,
        &options);

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
    format_html(root, &options, &mut output).expect("can format HTML");
    let output = String::from_utf8(output).expect("valid utf-8 generated HTML");
    Ok(output)
}

fn format_page<W: io::Write>(frontmatter: models::frontmatter::FrontMatter, chapters: &Vec<models::chapter::Chapter>, url: &str, content: &str, mut output: W) -> Result<(), Box<dyn std::error::Error>> {
    use askama::Template;
    #[derive(Template)]
    #[template(path = "page.html")]
    struct PageTemplate<'a, 'b, 'c, 'd, 'e, 'f> {
        title: &'a str,
        content: &'b str,
        url: &'f str,
        chapters: &'c Vec<models::chapter::Chapter>,
        prev_chapter: Option<&'d models::chapter::Chapter>,
        next_chapter: Option<&'e models::chapter::Chapter>,
    }

    let this_index = chapters.iter().enumerate().find(|(_, chap)| chap.url == url).map(|(i, _)| i).expect("chapter exists");
    let prev_chapter = if this_index > 0 {
        Some(chapters.iter().nth(this_index - 1).expect("chapter n-1 exists"))
    }
    else {
        None
    };
    let next_chapter = if this_index < chapters.len() - 1 {
        Some(chapters.iter().nth(this_index + 1).expect("chapter n+1 exists"))
    }
    else {
        None
    };

    // fill out our template
    let template = PageTemplate {
        title: &frontmatter.title,
        content,
        url,
        chapters,
        prev_chapter,
        next_chapter,
    };

    // and render!
    let s = template.render()?;
    output.write_all(s.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::build_cli().get_matches();

    if let Some(_submatches) = matches.subcommand_matches("init") {
        unimplemented!()
    }
    else if let Some(submatches) = matches.subcommand_matches("build") {
        let src = submatches.value_of("in").expect("in value");
        let dest = submatches.value_of("out").expect("out value");

        let src = PathBuf::from(src);
        let dest = PathBuf::from(dest);
        std::fs::create_dir_all(&dest)?;

        // load all our chapters
        let mut chapters: Vec<models::chapter::Chapter> = Vec::default();
        for entry in src.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if let Some("md") = path.extension().map(std::ffi::OsStr::to_str).flatten() {
                let name = path.file_stem().map(std::ffi::OsStr::to_str).flatten();
                if name.is_none() { continue; }
                let name = name.unwrap();

                let contents = fs::read_to_string(&path)?;
                let (front, _) = extract_frontmatter(&contents)?;
                let front = front.unwrap_or_default().into_front(name);
                chapters.push(models::chapter::Chapter {
                    url: format!("{}.html", name),
                    title: front.title,
                });
            }
        }
        chapters.sort_by(|a, b| a.url.cmp(&b.url));

        // compile markdown
        for entry in src.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if let Some("md") = path.extension().map(std::ffi::OsStr::to_str).flatten() {
                let name = path.file_stem().map(std::ffi::OsStr::to_str).flatten();
                if name.is_none() { continue; }
                let name = name.unwrap();
                let out = dest.join(format!("{}.html", name));
                
                let outfile = fs::File::create(&out)?;
                let outfile = io::BufWriter::new(outfile);

                let contents = fs::read_to_string(&path)?;
                let (front, contents) = extract_frontmatter(&contents)?;
                let front = front.unwrap_or_default().into_front(name);
                let contents = format_markdown(&contents)?;
                format_page(front, &chapters, &format!("{}.html", name), &contents, outfile)?;

                println!("Rendered `{}` into `{}`", path.display(), out.display());
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
