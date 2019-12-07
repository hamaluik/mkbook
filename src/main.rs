#[macro_use]
extern crate lazy_static;

use std::path::{Path, PathBuf};
use std::{fs};

pub const ASSET_DEFAULT_README: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/README.default.md"));
pub const ASSET_DEFAULT_INTRODUCTION: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/01-introduction.default.md"));


mod cli;
mod models;
mod html;
mod latex;
mod extensions;

use models::book::Book;
use models::chapter::Chapter;
use models::frontmatter::{ParsedFrontMatter, FrontMatter};

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

/// load the entire book at once
/// maybe a bad idea with large books but ¯\_(ツ)_/¯
fn load_book<P: AsRef<Path>>(src: P) -> Result<Book, Box<dyn std::error::Error>> {
    // load our book
    let src = PathBuf::from(src.as_ref());
    let book_readme_path = src.join("README.md");
    let (book_front, book_description) = if book_readme_path.exists() {
        let contents = fs::read_to_string(&book_readme_path)?;
        let (front, contents) = extract_frontmatter(&contents)?;
        (front, contents)
    }
    else {
        let content = String::new();
        (None, content)
    };
    let book_front = FrontMatter::from_root(book_front.unwrap_or_default());

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
                let (front, contents) = extract_frontmatter(&contents)?;
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
                    let (front, contents) = extract_frontmatter(&contents)?;
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
            let (front, contents) = extract_frontmatter(&contents)?;
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

    Ok(Book {
        front: book_front,
        description: book_description,
        chapters,
    })
}

struct ReloadClient {
    sender: std::sync::Arc<ws::Sender>,
    reload: std::sync::Arc<std::sync::atomic::AtomicBool>,
    quitloop: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl ReloadClient {
    pub fn new(sender: ws::Sender, reload: std::sync::Arc<std::sync::atomic::AtomicBool>) -> ReloadClient {
        ReloadClient {
            sender: std::sync::Arc::new(sender),
            reload,
            quitloop: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
}

impl ws::Handler for ReloadClient {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        log::info!("reload client connected");
        let out = self.sender.clone();
        let reload = self.reload.clone();
        let quitloop = self.quitloop.clone();
        std::thread::spawn(move || {
            'sendloop: loop {
                let send_reload = reload.load(std::sync::atomic::Ordering::SeqCst);
                if send_reload {
                    log::debug!("sending reload signal...");
                    out.send("reload").expect("can send reload signal");
                    log::debug!(" ok!");
                }

                let quit = quitloop.load(std::sync::atomic::Ordering::SeqCst);
                if quit {
                    break 'sendloop;
                }
                
                // check at 10 Hz
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            log::warn!("shutting down reload connection");
            if let Err(e) = out.shutdown() {
                log::error!("failed to shut down reload connection: {:?}", e);
            }
        });
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        log::debug!("reload connection closed: {:?}: {}", code, reason);
        //self.quitloop.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    fn on_shutdown(&mut self) {
        log::debug!("reload connection shutdown");
        self.quitloop.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::build_cli().get_matches();

    use fern::colors::{Color, ColoredLevelConfig};
    let colors_level = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Cyan)
        .debug(Color::Magenta)
        .trace(Color::BrightBlack);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}][\x1B[96m{target}\x1B[0m][{level}\x1B[0m] {message}",
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        .level_for("ws", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    if let Some(submatches) = matches.subcommand_matches("init") {
        let dest = submatches.value_of("directory").expect("directory value");
        let dest = PathBuf::from(dest);

        log::info!("Initializing a book into {}...", dest.display());
        fs::create_dir_all(&dest)?;
        let book_readme_path = dest.join("README.md");
        fs::write(&book_readme_path, ASSET_DEFAULT_README)?;
        let intro_path = dest.join("01-introduction.md");
        fs::write(&intro_path, ASSET_DEFAULT_INTRODUCTION)?;
        log::info!("Done!");

        log::info!("You can now build your book by running:");
        if dest.display().to_string() != "src" {
            log::info!("mkbook build -i {}", dest.display());
        }
        else {
            log::info!("mkbook build");
        }

        Ok(())
    }
    else if let Some(submatches) = matches.subcommand_matches("build") {
        let src = submatches.value_of("in").expect("in value");
        let dest = submatches.value_of("out").expect("out value");

        if submatches.is_present("latex") {
            let latex_file = submatches.value_of("latex").unwrap();
            let latex_file = PathBuf::from(latex_file);
            latex::build(src, latex_file)
        }
        else {
            html::build(src, dest, false)
        }
    }
    else if let Some(submatches) = matches.subcommand_matches("watch") {
        let reload_trigger = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let do_reload = submatches.is_present("reload");
        
        if do_reload {
            let reload_trigger = reload_trigger.clone();
            std::thread::spawn(move || {
                log::info!("starting livereload service");
                ws::listen("127.0.0.1:3456", |out| ReloadClient::new(out, reload_trigger.clone())).expect("can launch livereload service");
            });
        }

        use notify::{RecommendedWatcher, RecursiveMode, Watcher};

        let src = submatches.value_of("in").expect("in value");
        let dest = submatches.value_of("out").expect("out value");
        html::build(src, dest, do_reload)?;

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, std::time::Duration::from_secs(1))?;
        watcher.watch(src, RecursiveMode::Recursive)?;

        loop {
            match rx.recv() {
                Ok(notify::DebouncedEvent::NoticeWrite(_)) | Ok(notify::DebouncedEvent::NoticeRemove(_)) => {},
                Ok(_) => {
                    html::build(src, dest, do_reload)?;
                    reload_trigger.store(true, std::sync::atomic::Ordering::SeqCst);
                    std::thread::sleep(std::time::Duration::from_millis(150));
                    reload_trigger.store(false, std::sync::atomic::Ordering::SeqCst);
                },
                Err(e) => {
                    log::error!("watch error: {:?}", e);
                    return Err(Box::from(e));
                }
            }
        }
    }
    else {
        cli::build_cli().print_long_help()?;
        Ok(())
    }
}
