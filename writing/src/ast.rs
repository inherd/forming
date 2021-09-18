#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Writing {
    pub code_docs: Vec<DocCode>,
}

impl Writing {
    pub fn new() -> Writing {
        Writing {
            code_docs: vec![]
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocCode {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
}

impl DocCode {
    pub fn new() -> DocCode {
        DocCode {
            file: "".to_string(),
            start_line: 0,
            end_line: 0
        }
    }
}