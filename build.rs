fn main() {
    // compile our styles
    let in_dir = std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir is present");
    let in_dir = std::path::PathBuf::from(in_dir).join("style");
    let in_file = in_dir.join("style.scss");

    let options = sass_rs::Options {
        output_style: sass_rs::OutputStyle::Compressed,
        ..sass_rs::Options::default()
    };
    let compiled = sass_rs::compile_file(&in_file, options).expect("can compile styles");

    let out_dir = std::env::var("OUT_DIR").expect("out dir is present");
    let out_dir = std::path::PathBuf::from(out_dir);
    let out_file = out_dir.join("style.css");
    std::fs::write(&out_file, compiled).expect("can write stylesheet");
}