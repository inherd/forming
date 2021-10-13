#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptSpace {
    name: String,
    package: String,
    concepts: Vec<Concepts>,
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Concepts {
    name: String,
    structs: Vec<Struct>,
    behaviors: Vec<Function>,
    functions: Vec<Function>,
}

impl Concepts {
    pub fn new(name: String) -> Concepts {
        Concepts { name, structs: vec![], behaviors: vec![], functions: vec![] }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Struct {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Behavior {}

/// auto insert/update comment to code
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighlightCore {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeType {
    Int,
    Float,
    Double,
    String,
    Array,
    Type,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiResource {
    name: String,
    base_url: String,
    source: String,
    api: Vec<ApiDecl>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiDecl {
    input: Vec<Parameter>,
    output: Vec<Parameter>,
    pre_condition: String,
    post_condition: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    name: String,
    typ: TypeType,
}