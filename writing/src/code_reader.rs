use std::fs::File;
use std::io::{BufRead, BufReader};

use guarding_core::domain::code_file::CodeFile;
use guarding_core::domain::code_function::CodeFunction;
use guarding_ident::ModelBuilder;

use crate::ast::{CodeFunc, CodeSection, CodeSource};

pub struct CodeReader {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Point {
    start: usize,
    end: usize,
}

impl CodeReader {
    pub fn read_doc_code(doc: &CodeSource) -> Vec<String> {
        let start = doc.start_line;
        let end = doc.end_line;

        let lines = CodeReader::file_to_lines(&doc.file);
        CodeReader::read_by_position(&lines, start, end)
    }

    fn read_by_position(lines: &Vec<String>, start: usize, end: usize) -> Vec<String> {
        lines
            .into_iter()
            .enumerate()
            .filter(|(i, _l)| {
                i >= &(start - 1) && i < &end
            })
            .map(|(_i, l)| l.to_string())
            .collect()
    }

    pub fn read_code_func(doc: &CodeFunc) -> Vec<String> {
        let mut str: Vec<String> = vec![];

        let mut models: Vec<CodeFile> = vec![];
        ModelBuilder::build_model_by_file(&mut models, doc.file.as_ref());
        let mut points = vec![];
        for model in &models {
            for clz in &model.classes {
                let funcs = &clz.functions;
                CodeReader::filter_by_func(&doc, &mut points, funcs);
            }

            let funcs = &model.functions;
            CodeReader::filter_by_func(&doc, &mut points, funcs);
        }

        let path = &doc.file;
        let lines = CodeReader::file_to_lines(path);

        for point in &points {
            let mut text = CodeReader::read_by_position(&lines, point.start, point.end);
            str.append(&mut text);
        }

        str
    }

    pub fn file_to_lines(path: &String) -> Vec<String> {
        let file = File::open(path).expect("cannot read file");
        let lines: Vec<String> = BufReader::new(file)
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();

        lines
    }

    fn filter_by_func(doc: &&CodeFunc, points: &mut Vec<Point>, funcs: &Vec<CodeFunction>) {
        for func in funcs {
            for exp in &doc.funcs {
                if &func.name == exp {
                    points.push(Point {
                        start: func.start.row + 1,
                        end: func.end.row + 1,
                    })
                }
            }
        }
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
