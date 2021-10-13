use std::path::PathBuf;
use walkdir::{WalkDir};

pub struct ConceptScanner {

}

impl ConceptScanner {
    pub fn from_dir(dir: String, suffix: &str) -> Vec<PathBuf> {
        let buf = PathBuf::from(dir);
        ConceptScanner::scan_by_dir(buf, suffix)
    }

    fn scan_by_dir(buf: PathBuf, suffix: &str) -> Vec<PathBuf> {
        let walk_dir = WalkDir::new(buf);
        let files = walk_dir.into_iter().map(|entry| entry.expect("error dir"))
            .filter(|entry| {
                entry.path().ends_with(suffix)
            })
            .map(|entry| entry.into_path())
            .collect::<Vec<PathBuf>>();

        files
    }

    pub fn from_files(_path: String) {}

    // scan from files
    pub fn scan() {}
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::codegen::concept_scanner::ConceptScanner;

    #[test]
    fn scan_by_dir() {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src");

        println!("{:?}", root_dir.display());
        let files = ConceptScanner::scan_by_dir(root_dir, ".rs");
        println!("{:?}", files);
    }
}