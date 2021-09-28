#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Writing {
    pub code_docs: Vec<CodeSource>,
    pub code_deps: Vec<CodeDep>,
    pub code_sections: Vec<CodeSection>,
    pub code_funcs: Vec<CodeFunc>
}

impl Writing {
    pub fn new() -> Writing {
        Writing {
            code_docs: vec![],
            code_deps: vec![],
            code_sections: vec![],
            code_funcs: vec![]
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeSource {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
}

impl CodeSource {
    pub fn new() -> CodeSource {
        CodeSource {
            file: "".to_string(),
            start_line: 0,
            end_line: 0
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeDep {
    pub name: String,
    pub version: String,
    pub artifact_id: String,
    pub group_id: String,
}

impl CodeDep {
    pub fn new() -> CodeDep {
        CodeDep {
            name: "".to_string(),
            version: "".to_string(),
            artifact_id: "".to_string(),
            group_id: "".to_string()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeSection {
    pub blocks: Vec<CodeBlock>
}

impl CodeSection {
    pub fn new() -> CodeSection {
        CodeSection {
            blocks: vec![]
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeBlock {
    pub file: String,
    pub name: String,
}

impl CodeBlock {
    pub fn new() -> CodeBlock {
        CodeBlock {
            file: "".to_string(),
            name: "".to_string()
        }
    }
}

// todo: thinking in polymorphism
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeFunc {
    pub file: String,
    pub funcs: Vec<String>,
}

impl CodeFunc {
    pub fn new() -> CodeFunc {
        CodeFunc {
            file: "".to_string(),
            funcs: vec![]
        }
    }
}

