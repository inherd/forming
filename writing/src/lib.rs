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
    use pulldown_cmark::{Options, Parser};
    use super::*;

    #[test]
    fn should_parse_line() {
        let string = String::from_utf8_lossy(&fs::read("README.md").unwrap()).to_string();

        let parser = Parser::new_ext(&*string, Options::empty());

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\nHTML output:\n").unwrap();
        md_writer::write_text(&mut handle, parser).unwrap();
    }

    #[test]
    fn should_convert_text() {
        let string = "
233333

```rust
// doc-code: file(\"src/lib.rs\").line()[2, 4]
// doc-code: file(\"src/lib.rs\").line()[4, 5]
```

[a link](dx.phodal.com)


demo
";

        let parser = Parser::new_ext(&*string, Options::empty());

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\nHTML output:\n").unwrap();
        md_writer::write_text(&mut handle, parser).unwrap();
    }
}