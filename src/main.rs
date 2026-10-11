use std::{env, fs, path::PathBuf};

use crate::parsing::parse_file;

mod parsing;

fn main() {
    let mut args = env::args().skip(1);

    let input = args.next().expect("Usage: bevy_vsg <file>");

    if input.ends_with(".wesl") {
        panic!("Error: File already ends with `.wgsl`, cannot overwrite a file.")
    }

    let file = fs::read(&input).unwrap();
    let mut path: PathBuf = input.into();
    path.set_extension("wesl");

    let content = std::str::from_utf8(&file).unwrap();

    let output = parse_file(content);

    fs::write(path, output).unwrap();
}
