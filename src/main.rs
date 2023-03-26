use std::collections::HashMap;
use std::ffi::OsString;
use std::io::Result;
use std::path::Path;
use std::str::Chars;
use std::{env, fs};

fn main() -> Result<()> {
    let args = env::args();
    if args.len() <= 1 {
        let current_dir = env::current_dir()?;
        let files_analyze = visit_dirs(current_dir.as_path())?;
        for (file_name, file_content) in files_analyze.iter() {
            if let Some(file_name) = file_name.to_str() {
                print_file_info(file_name, file_content)
            }
        }
        return Ok(());
    }
    let mut args = args.skip(1);
    if let Some(file_name) = args.next() {
        let file_content = analyze_file(Path::new(&file_name))?;
        print_file_info(&file_name, &file_content);
    }
    Ok(())
}

fn print_file_info(file_name: &str, file_content: &FileContent) {
    println!(
        "File \"{file_name}\" contains {} characters where {} are whitespace characters",
        file_content.length, file_content.whitespaces
    )
}

struct FileContent {
    length: usize,
    whitespaces: usize,
}

fn visit_dirs(path: &Path) -> Result<HashMap<OsString, FileContent>> {
    let mut file_info = HashMap::new();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let rec = visit_dirs(&path)?;
                file_info.extend(rec);
            }
            if path.is_file() {
                if let Ok(file_analyze) = analyze_file(path.as_path()) {
                    if let Some(file_name) = path.file_name() {
                        file_info.insert(file_name.to_owned(), file_analyze);
                    }
                }
            }
        }
    }
    Ok(file_info)
}

fn analyze_file(path: &Path) -> Result<FileContent> {
    match fs::read_to_string(path) {
        Ok(file) => {
            let file_content = file.chars();
            let whitespaces = count_whitespaces(file_content.clone());
            let length = file_content.count();
            Ok(FileContent {
                length,
                whitespaces,
            })
        }
        Err(e) => Err(e),
    }
}

fn count_whitespaces(chars: Chars) -> usize {
    chars.fold(
        0usize,
        |acc, e| if e.is_whitespace() { acc + 1 } else { acc },
    )
}

#[cfg(test)]
mod tests {
    use crate::count_whitespaces;

    #[test]
    fn return_0_on_emptyword() {
        let word = "".chars();
        assert_eq!(count_whitespaces(word), 0);
    }
    #[test]
    fn returns_expected_result_on_only_whitespace_chars() {
        let word = " \t\n\r".chars();
        assert_eq!(count_whitespaces(word), 4);
    }

    #[test]
    fn assert_null_isnt_whitespace() {
        let word = "\0\0\0".chars();
        assert_eq!(count_whitespaces(word), 0);
    }
}
