#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceUnitPart {
    Architecture(Architecture),
    StructUnit(StructUnit),
    Concept(Concept),
    ConceptSource(ConceptSource),
    Contract(Contract),
    Api(ApiRoot),
    Behavior(Behavior),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Architecture {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptSource {
    pub cataloging: Cataloging,
    pub path: String,
}

impl ConceptSource {
    pub fn new() -> ConceptSource {
        ConceptSource {
            cataloging: Cataloging::File,
            path: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Cataloging {
    File,
    Dir,
}

impl Cataloging {
    pub fn from(text: String) -> Cataloging {
        match text.as_str() {
            "file" => {
                Cataloging::File
            }
            "dir" => {
                Cataloging::Dir
            }
            _ => {
                Cataloging::File
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptSpace {
    identifier: String,
    package: String,
    concepts: Vec<Concept>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiRoot {
    pub name: String,
    pub import: Option<String>,
    pub apis: Vec<ApiNode>,
}

impl ApiRoot {
    pub fn new() -> ApiRoot {
        ApiRoot {
            name: "".to_string(),
            import: None,
            apis: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiNode {
    pub api_in: Vec<StructField>,
    pub api_out: Vec<StructField>,
    pub pre_cond: String,
    pub post_cond: String,
}

impl ApiNode {
    pub fn new() -> ApiNode {
        ApiNode {
            api_in: vec![],
            api_out: vec![],
            pre_cond: "".to_string(),
            post_cond: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    text: String,
    expr: Expression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    String(String)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Concept {
    identifier: String,
    structs: Vec<StructUnit>,
    behaviors: Vec<Function>,
    functions: Vec<Function>,
}

impl Concept {
    pub fn new(identifier: String) -> Concept {
        Concept { identifier, structs: vec![], behaviors: vec![], functions: vec![] }
    }
}

// naming refs: https://github.com/vickenty/lang-c/blob/master/grammar.rustpeg
// naming refs: https://github.com/vickenty/lang-c
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructUnit {
    pub identifier: String,
    pub declarations: Vec<StructField>,
}

impl StructUnit {
    pub fn new() -> StructUnit {
        StructUnit {
            identifier: "".to_string(),
            declarations: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructField {
    pub identifier: String,
    pub declarator: TypeSpecifier,
}

impl StructField {
    pub fn new() -> StructField {
        StructField {
            identifier: "".to_string(),
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
