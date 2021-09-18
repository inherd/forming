#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Writing {
    pub code_docs: Vec<CodeDoc>,
}

impl Writing {
    pub fn new() -> Writing {
        Writing {
            code_docs: vec![]
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeDoc {
    pub file: String,
    pub start_line: i32,
    pub end_line: i32,
}

impl CodeDoc {
    pub fn new() -> CodeDoc {
        CodeDoc {
            file: "".to_string(),
            start_line: 0,
            end_line: 0
        }
    }
}