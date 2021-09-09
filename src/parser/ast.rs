#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptSpace {
    name: String,
    package: String,
    concepts: Vec<Concepts>
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Concepts {
    name: String,
    structs: Vec<Struct>,
    behaviors: Vec<Function>,
    functions: Vec<Function>
}

impl Concepts {
    pub fn new(name: String) -> Concepts {
        Concepts { name, structs: vec![], behaviors: vec![], functions: vec![] }
    }

    pub fn from_dir(dir: String) {

    }

    pub fn from_files(path: String) {

    }

    // scan from files
    pub fn scan() {

    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Struct {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Behavior {

}

/// auto insert/update comment to code
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighlightCore {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Int,
    Float,
    Double,
    String,
    Array,
    Type
}

