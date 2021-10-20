use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::parser::ast::{CodeSection, CodeSource};

pub struct CodeReader {}

impl CodeReader {
    pub fn read_doc_code(doc: &CodeSource) -> Vec<String> {
        let file = File::open(&doc.file).expect("cannot read file");
        let reader = BufReader::new(file);

        reader.lines()
            .enumerate()
            .filter(|(i, _l)| {
                i >= &(doc.start_line - 1) && i < &doc.end_line
            })
            .map(|(_i, l)| l.expect("cannot parse"))
            .collect()
    }

    pub fn read_doc_section(doc: &CodeSection) -> Vec<String> {
        let file = File::open(&doc.blocks[0].file).expect("cannot read file");
        let reader = BufReader::new(file);

        let mut is_during = false;
        let mut str: Vec<String> = reader.lines()
            .map(|l| l.expect("cannot parse"))
            .filter(|text| {
                if text.ends_with("doc-start: section1") {
                    is_during = true;
                }
                if text.ends_with("doc-end: section1") {
                    is_during = false;
                }

                is_during
            })
            .collect();

        str.remove(0);

        str
    }
}
