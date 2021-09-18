use crate::ast::DocCode;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct WReader {

}

impl WReader {
    pub fn read_doc_code(doc: DocCode) -> Vec<String> {
        let file = File::open(doc.file).expect("cannot read file");
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        lines[doc.start_line..doc.end_line].to_owned()
    }
}

