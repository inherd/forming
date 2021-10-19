extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::wmd::Wmd;

pub mod parser;
pub mod wmd;
pub mod wreader;
pub mod md_writer;

#[derive(Error, Debug)]
pub enum WritingError {
    #[error("io error: `{0}` ")]
    IOError(String),
    #[error("unknown data store error")]
    Unknown,
}

pub struct Writing {}

impl Writing {
    pub fn process_file<P: AsRef<Path>>(path: P)  -> Result<String, WritingError> {
        let path = path.as_ref().to_path_buf();

        match Writing::pre_process_file(&path) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        let text = String::from_utf8_lossy(&fs::read(path).unwrap()).to_string();
        let mut wmd = Wmd::new(text);
        let result = wmd.parse();

        Ok(result)
    }

    fn pre_process_file(path: &PathBuf) -> Result<(), WritingError> {
        if path.is_dir() {
            return Err(WritingError::IOError(format!("path: {:?} is a dir", path)))
        }

        if let Err(e) = fs::read(path) {
            return Err(WritingError::IOError(format!("read file error: {:?}", e)))
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::io::Write;
    use pulldown_cmark::{Event, Options, Parser, Tag};
    use super::*;

    #[test]
    fn should_parse_line() {
        // let result = Writing::process_file("README.md");
        let string = String::from_utf8_lossy(&fs::read("README.md").unwrap()).to_string();

        let parser = Parser::new_ext(&*string, Options::empty())
            .map(|event| match event {
                Event::Text(text) => Event::Text(text.replace("Peter", "John").into()),
                _ => event,
            })
            .filter(|event| match event {
                Event::Start(Tag::Image(..)) | Event::End(Tag::Image(..)) => false,
                _ => true,
            });

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\nHTML output:\n").unwrap();
        md_writer::write_text(&mut handle, parser).unwrap();
    }
}