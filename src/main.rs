use std::path::Path;

mod vm;
mod parser;
mod optimizer;
mod driver;

fn main() {
    let mut args = std::env::args();
    let path = args.next().expect("Compiler's path not found");
    let src_path = match args.next() {
        Some(src) => src,
        _ => {
            println!("usage: {} [source file]", path);
            return;
        }
    };

    let mut src = match std::fs::File::open(&src_path) {
        Ok(file) => file,
        Err(e) => {
            println!("cannot open {}: {}", src_path, e);
            return;
        }
    };

    let output_path = Path::new(&src_path).file_stem().unwrap();
    let mut out = match std::fs::File::create(output_path) {
        Ok(file) => file,
        Err(e) => {
            println!("cannot create output file: {}", e);
            return;
        }
    };

    driver::compile_x86(&mut src, &mut out);
}
