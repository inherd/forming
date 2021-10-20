extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use std::path::{Path, PathBuf};

use pulldown_cmark::{Options, Parser};
use thiserror::Error;

pub mod parser;
pub mod md_reader;
pub mod md_writer;

#[derive(Error, Debug)]
pub enum WritingError {
    #[error("io error: `{0}` ")]
    IOError(String),
    #[error("read file error: `{0}` ")]
    ReadFileError(String),
    #[error("unknown data store error")]
    Unknown,
}

pub struct Writing {}

impl Writing {
    pub fn process_file<P: AsRef<Path>>(path: P)  -> Result<String, WritingError> {
        let path = path.as_ref().to_path_buf();

        let blob = match Writing::pre_process_file(&path) {
            Ok(s) => s,
            Err(err) => {
                return Err(err);
            }
        };

        let text = String::from_utf8_lossy(&blob).to_string();

        let parser = Parser::new_ext(&*text, Options::all());
        let mut result: String = String::from("");

        md_writer::push_text(&mut result, parser);

        Ok(result)
    }

    fn pre_process_file(path: &PathBuf) -> Result<Vec<u8>, WritingError> {
        if path.is_dir() {
            return Err(WritingError::IOError(format!("path: {:?} is a dir", path)))
        }

        if let Err(e) = fs::read(path) {
            return Err(WritingError::IOError(format!("read file error: {:?}", e)))
        }

        let blob = match fs::read(&path) {
            Ok(s) => s,
            Err(e) => return Err(WritingError::ReadFileError(format!("error: {:?}", &e))),
        };

        Ok(blob)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;

    use pulldown_cmark::{Options, Parser};

    use super::*;

    // doc-start: section1
    #[test]
    fn should_parse_line() {
        let string = String::from_utf8_lossy(&fs::read("README.md").unwrap()).to_string();

        let parser = Parser::new_ext(&*string, Options::empty());

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\nHTML output:\n").unwrap();
        md_writer::write_text(&mut handle, parser).unwrap();
    }
    // doc-end: section1
}
