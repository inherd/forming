#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptSpace {
    identifier: String,
    package: String,
    concepts: Vec<Concepts>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Concepts {
    identifier: String,
    structs: Vec<Struct>,
    behaviors: Vec<Function>,
    functions: Vec<Function>,
}

impl Concepts {
    pub fn new(identifier: String) -> Concepts {
        Concepts { identifier, structs: vec![], behaviors: vec![], functions: vec![] }
    }
}

// naming refs: https://github.com/vickenty/lang-c/blob/master/grammar.rustpeg
// naming refs: https://github.com/vickenty/lang-c
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Struct {
    identifier: String,
    declarations: Vec<StructDecl>
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructDecl {
    specifier: String,
    declarator: TypeSpecifier
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Behavior {
    identifier: String,
    functions: Vec<Interface>
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Interface {
    identifier: String,
    params: Vec<Parameter>,
    return_type: TypeSpecifier
}

/// auto insert/update comment to code
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighlightCore {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeSpecifier {
    Int,
    Float,
    Double,
    String,
    Array,
    Type,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiResource {
    identifier: String,
    base_url: String,
    source: String,
    api: Vec<ApiDecl>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiDecl {
    inbound: Vec<Parameter>,
    outbound: Vec<Parameter>,
    pre_condition: String,
    post_condition: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    identifier: String,
    specifier: TypeSpecifier,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contract {
    name: String,
    during: String,
    pre_condition: String,
    post_condition: String,
}
