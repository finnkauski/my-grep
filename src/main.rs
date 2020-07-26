#![feature(try_trait)]

use std::option::NoneError;

mod args;

mod io {
    use std::fs;

    type Filepath<'a> = &'a str;

    pub fn get_file(filepath: Filepath) -> Option<String> {
        fs::read_to_string(filepath)
            .map_err(|_| println!("Could not load file: {}", filepath))
            .ok()
    }
}

fn search(pattern: &str, contents: String) -> bool {
    contents.contains(pattern)
}

fn proc(args: args::Args) -> Vec<bool> {
    let pattern = args.pattern;
    let process = |f: String| -> bool {
        let contents = io::get_file(&f).unwrap();
        search(&pattern, contents)
    };
    args.files.map(process).collect()
}
fn main() {
    println!("{:?}", args::get_args().map(proc));
}
