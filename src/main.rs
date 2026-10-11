use std::{env, fs, path::PathBuf, process::ExitCode};

use crate::parsing::parse_file;

mod parsing;

fn main() -> ExitCode {
    macro_rules! abort {
        ($($tt: tt)*) => {{
            eprintln!($($tt)*);
            return ExitCode::FAILURE;
        }};
    }

    let mut args = env::args().skip(1);

    let Some(input) = args.next() else {
        abort!("Usage: bevy_vsg <file>");
    };

    let None = args.next() else {
        abort!("Usage: bevy_vsg <file>");
    };

    if input.ends_with(".wesl") {
        abort!("Error: File already ends with `.wgsl`, cannot overwrite the file.");
    }

    let file = match fs::read(&input) {
        Ok(file) => file,
        Err(e) => abort!("Error: {e}"),
    };

    let mut path: PathBuf = input.into();
    path.set_extension("wesl");

    let Ok(content) = std::str::from_utf8(&file) else {
        eprintln!("Error: Expected valid utf8.");
        return ExitCode::FAILURE;
    };

    let output = match parse_file(content) {
        Ok(output) => output,
        Err(e) => abort!("{e}"),
    };

    if let Err(err) = fs::write(path, output) {
        abort!("Error: {err}");
    }

    ExitCode::SUCCESS
}
