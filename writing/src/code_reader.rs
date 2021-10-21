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
        let block = &doc.blocks[0];
        let file = File::open(&block.file).expect("cannot read file");
        let reader = BufReader::new(file);

        let start_text = format!("doc-start: {}", block.name);
        let end_text = format!("doc-end: {}", block.name);

        let mut is_during = false;
        let mut str: Vec<String> = reader.lines()
            .map(|l| l.expect("cannot parse"))
            .filter(|text| {
                if text.ends_with(&start_text) {
                    is_during = true;
                }
                if text.ends_with(&end_text) {
                    is_during = false;
                }

                is_during
            })
            .collect();

        if str.len() > 0 {
            str.remove(0);
        }

        str
    }
}
