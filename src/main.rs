#![feature(try_trait)]
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
mod processing {
    use crate::{args, io};
    type SearchResults = Vec<String>;

    pub(crate) fn search(filename: &str, pattern: &str, contents: String) -> SearchResults {
        contents
            .lines() // get lines
            .enumerate() // get line numbers
            .filter(|(_, l)| l.contains(pattern)) // remove the ones that don't have the pattern
            .map(|(idx, l)| format_line(filename, idx, l)) // turn indices and lines into formatted output
            .collect() // collect into vector
    }

    /// Formats the final output
    pub(crate) fn format_line(filename: &str, idx: usize, line: &str) -> String {
        format!("{}::{}: {}", filename, idx, line)
    }

    /// Takes the parsed arguments and does the magic
    pub fn proc(args: args::Args) -> Vec<Option<SearchResults>> {
        let pattern = args.pattern;
        let process = |f: &String| -> Option<SearchResults> {
            io::get_file(&f)
                .map(|content| search(&f, &pattern, content)) // Get resulting vector TODO: remove passing f?
                .filter(|vec| !vec.is_empty()) // Returns None if empty
        };
        args.files.iter().map(process).collect()
    }
}

fn main() {
    let results = args::get_args().map(processing::proc).unwrap();
    results.iter().for_each(|op_result| {
        if let Some(result) = op_result {
            result.iter().for_each(|line| println!("{}", line));
        }
    })
}

#[cfg(test)]
mod tests {
    use super::processing::*;
    use super::*;
    #[test]
    fn test_search_found() {
        // This should return one line
        let contents = "testingline1\ntestingline2\ntestingFOOBARline3";
        let pattern = "FOOBAR";
        assert_eq!(
            search(pattern, String::from(contents)),
            vec![format_line(2, "testingFOOBARline3")]
        );
    }
    #[test]
    fn test_search_found_several() {
        // This should return all lines
        let contents = "testingline1\ntestingline2\ntestingFOOBARline3";
        let pattern = "testing";
        let all_formated_lines = contents
            .lines()
            .enumerate()
            .map(|(idx, l)| format_line(idx, l))
            .collect::<Vec<String>>();
        let result = search(pattern, String::from(contents));

        assert!(result.len() == 3); // Trivial but meh
        assert_eq!(result, all_formated_lines); // Check that we get all lines back
    }
    #[test]
    fn test_search_notfound() {
        let contents = "testingline1\ntestingline2\ntestingFOOBARline3";
        let pattern = "BARFOO";
        let result = search(pattern, String::from(contents));
        assert_eq!(result, Vec::<String>::new());
    }
}
