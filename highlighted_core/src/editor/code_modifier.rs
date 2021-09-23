use fnv::FnvHashMap;
use std::io::{Read, Error, Seek, SeekFrom, ErrorKind};

pub struct CodeModifier<R> {
    file: R,
    file_size: u64,
    newline_map: FnvHashMap<usize, usize>,
}

impl<R: Read + Seek> CodeModifier<R> {
    pub fn new(mut file: R) -> Result<Self, Error> {
        let file_size = file.seek(SeekFrom::End(0))?;
        if file_size == 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "Empty file"));
        }

        Ok(CodeModifier {
            file,
            file_size,
            newline_map: Default::default()
        })
    }
}



#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;
    use crate::editor::code_modifier::CodeModifier;

    #[test]
    fn should_open_file() {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let file = File::open(d).unwrap();
        let mut reader = CodeModifier::new(file).unwrap();
    }
}