use std::{fs::read_to_string, process::ExitCode};
use strinject::*;

fn main() -> ExitCode {
    let args = std::env::args().collect::<Vec<_>>();
    let input_path = args
        .get(1)
        .expect("Expected 1 argument: the path to the file to parse.");
    let file: &str = &read_to_string(input_path)
        .unwrap_or_else(|_| panic!("Could not read file at input {}", input_path));

    let result = inject_with_path(file, |file_path| {
        let mut path = "..".to_string();
        path.push_str(file_path);
        path.to_string()
    });
    match result {
        Ok(result) => {
            print!("{result}");
        }
        Err(error) => {
            for e in error.errors {
                match e {
                    ErrorType::IncorrectTag => {
                        eprintln!("❌ ERROR: a <load> tag was incorrectly written in {}, maybe missing path or marker, or not using `'` for string delimiters ?", input_path);
                    }
                    ErrorType::IncorrectPath(path) => {
                        eprintln!("❌ ERROR: path {} not found", path);
                    }
                    ErrorType::IncorrectMarker(IncorrectMarker { marker, filepath }) => {
                        eprintln!("❌ ERROR: marker {} not found in {}", marker, filepath);
                    }
                }
            }
            if let Some(result) = error.result {
                print!("{result}");
            }
            return ExitCode::from(1);
        }
    }
    ExitCode::from(0)
}
