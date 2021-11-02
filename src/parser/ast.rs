#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptSpace {
    identifier: String,
    package: String,
    concepts: Vec<Concepts>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiRoot {
    name: String,
    import: String,
    apis: Vec<ApiNode>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiNode {
    pub api_in: Option<StructNode>,
    pub api_out: Option<StructNode>,
    pub pre_cond: String,
    pub post_cond: String,
}

impl ApiNode {
    pub fn new() -> ApiNode {
        ApiNode {
            api_in: None,
            api_out: None,
            pre_cond: "".to_string(),
            post_cond: "".to_string()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    text: String,
    expr: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Concepts {
    identifier: String,
    structs: Vec<StructNode>,
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
pub struct StructNode {
    pub identifier: String,
    pub declarations: Vec<StructDecl>,
}

impl StructNode {
    pub fn new() -> StructNode {
        StructNode {
            identifier: "".to_string(),
            declarations: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructDecl {
    pub specifier: String,
    pub declarator: TypeSpecifier,
}

impl StructDecl {
    pub fn new() -> StructDecl {
        StructDecl {
            specifier: "".to_string(),
            declarator: TypeSpecifier::TypeType(String::from("")),
        }
    }
    pub fn parse_type(text: String) -> TypeSpecifier {
        match text.to_lowercase().as_str() {
            "int" => {
                TypeSpecifier::Int
            }
            "float" => {
                TypeSpecifier::Float
            }
            "double" => {
                TypeSpecifier::Double
            }
            "string" => {
                TypeSpecifier::String
            }
            "array" => {
                TypeSpecifier::Array
            }
            _ => {
                TypeSpecifier::TypeType(text)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Behavior {
    identifier: String,
    functions: Vec<Interface>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Interface {
    identifier: String,
    params: Vec<Parameter>,
    return_type: TypeSpecifier,
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
    TypeType(String),
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
