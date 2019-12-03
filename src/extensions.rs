pub fn create_katex_inline(src: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    let mut child = match Command::new("katex")
        .arg("-d")
        .arg("-t")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() {
            Ok(c) => c,
            Err(e) => {
                log::warn!("failed to launch katex, not rendering math block: {:?}", e);
                return Err(Box::from(e));
            }
        };

    let stdin = child.stdin.as_mut().expect("valid katex stdin");
    stdin.write_all(src.as_ref())?;

    let output = child.wait_with_output()?;
    if !output.status.success() {
        log::error!("failed to generate katex, exit code: {:?}", output.status.code());
        log::error!("katex STDOUT:");
        log::error!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        log::error!("katex STDERR:");
        log::error!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        log::error!("/katex output");
        return Err(Box::from("katex failed"));
    }
    let rendered: String = String::from_utf8(output.stdout)?;

    Ok(format!(r#"<figure class="math">{}</figure>"#, rendered))
}

pub fn create_plantuml_svg(src: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    let mut child = match Command::new("plantuml")
        .arg("-tsvg")
        .arg("-nometadata")
        .arg("-pipe")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() {
            Ok(c) => c,
            Err(e) => {
                log::warn!("failed to launch plantuml, not rendering plantuml block: {:?}", e);
                return Err(Box::from(e))
            }
        };

    let stdin = child.stdin.as_mut().expect("valid plantuml stdin");
    stdin.write_all(src.as_ref())?;

    let output = child.wait_with_output()?;
    if !output.status.success() {
        log::error!("failed to generate plantuml, exit code: {:?}", output.status.code());
        log::error!("plantuml STDOUT:");
        log::error!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        log::error!("plantuml STDERR:");
        log::error!("{}", String::from_utf8_lossy(output.stdout.as_ref()));
        log::error!("/plantuml output");
        return Err(Box::from("plantuml failed"));
    }
    let svg: String = String::from_utf8(output.stdout)?;
    let svg = svg.replace(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#, "");

    Ok(format!("<figure>{}</figure>", svg))
}