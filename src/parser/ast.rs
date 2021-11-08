#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceUnitPart {
    Architecture(ArchitectureDecl),
    ApiUnit(ApiDecl),
    Contract(ContractDecl),
    Concept(ConceptDecl),
    ConceptSpace(ConceptSpaceDecl),
    ConceptBy(ConceptByDecl),
    StructFor(StructForDecl),
    BehaviorFor(BehaviorForDecl),
    Diagram(Diagram),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchitectureDecl {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagram {
    pub identify: String,
    pub groups: Vec<DiagramGroup>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagramGroup {
    pub items: Vec<DiagramItem>,
    pub subs: Vec<DiagramGroup>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagramItem {
    pub identify: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptByDecl {
    pub cataloging: Cataloging,
    pub path: String,
}

impl ConceptByDecl {
    pub fn new() -> ConceptByDecl {
        ConceptByDecl {
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
            "file" => Cataloging::File,
            "dir" => Cataloging::Dir,
            _ => Cataloging::File
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptSpaceDecl {
    pub identifier: String,
    pub package: String,
    pub type_type: String,
    pub concepts: Vec<String>,
}

impl ConceptSpaceDecl {
    pub fn new() -> ConceptSpaceDecl {
        ConceptSpaceDecl {
            identifier: "".to_string(),
            package: "".to_string(),
            type_type: "".to_string(),
            concepts: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiDecl {
    pub identify: String,
    pub url: String,
    pub import: Option<String>,
    pub apis: Vec<ApiNode>,
}

impl ApiDecl {
    pub fn new() -> ApiDecl {
        ApiDecl {
            identify: "".to_string(),
            url: "".to_string(),
            import: None,
            apis: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiNode {
    pub identify: String,
    pub inbound: Vec<StructField>,
    pub outbound: Vec<StructField>,
    pub pre_cond: Vec<ConditionDecl>,
    pub post_cond: Vec<ConditionDecl>,
}

impl ApiNode {
    pub fn new() -> ApiNode {
        ApiNode {
            identify: "".to_string(),
            inbound: vec![],
            outbound: vec![],
            pre_cond: vec![],
            post_cond: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConditionDecl {
    pub text: String,
    pub expr: Expression,
}

impl ConditionDecl {
    pub fn new() -> ConditionDecl {
        ConditionDecl {
            text: "".to_string(),
            expr: Expression::String(String::from("")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    String(String)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptDecl {
    pub description: String,
    pub identifier: String,
    pub extends: Vec<String>,
    pub behaviors: Vec<Behavior>,
    pub struct_fields: Vec<StructField>,
    pub struct_source: StructSource,
}

impl ConceptDecl {
    pub fn new() -> ConceptDecl {
        ConceptDecl { description: "".to_string(), identifier: "".to_string(), extends: vec![], struct_fields: vec![], behaviors: vec![], struct_source: Default::default() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructSource {
    path: String,
    source: String,
    type_type: String,
}

impl Default for StructSource {
    fn default() -> Self {
        StructSource {
            path: "".to_string(),
            source: "".to_string(),
            type_type: "".to_string(),
        }
    }
}

// naming refs: https://github.com/vickenty/lang-c/blob/master/grammar.rustpeg
// naming refs: https://github.com/vickenty/lang-c
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructForDecl {
    pub identifier: String,
    pub declarations: Vec<StructField>,
}

impl StructForDecl {
    pub fn new() -> StructForDecl {
        StructForDecl {
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BehaviorForDecl {
    pub identifier: String,
    pub behaviors: Vec<Behavior>,
}

impl BehaviorForDecl {
    pub fn new() -> BehaviorForDecl {
        BehaviorForDecl {
            identifier: "".to_string(),
            behaviors: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Behavior {
    pub params: Vec<Parameter>,
    pub identifier: String,
    pub return_type: TypeSpecifier,
}

impl Behavior {
    pub fn new() -> Behavior {
        Behavior {
            params: vec![],
            identifier: "".to_string(),
            return_type: TypeSpecifier::None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeSpecifier {
    /// no return or empty
    None,
    Int,
    Float,
    Double,
    String,
    Array,
    TypeType(String),
}

impl TypeSpecifier {
    pub fn from(text: String) -> TypeSpecifier {
        match text.to_lowercase().as_str() {
            "int" => TypeSpecifier::Int,
            "float" => TypeSpecifier::Float,
            "double" => TypeSpecifier::Double,
            "string" => TypeSpecifier::String,
            "array" => TypeSpecifier::Array,
            _ => TypeSpecifier::TypeType(text)
        }
    }
}
//
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct ApiDecl {
//     inbound: Vec<Parameter>,
//     outbound: Vec<Parameter>,
//     pre_condition: String,
//     post_condition: String,
// }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    pub identifier: String,
    pub specifier: TypeSpecifier,
}

impl Parameter {
    pub fn new() -> Parameter {
        Parameter {
            identifier: "".to_string(),
            specifier: TypeSpecifier::None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractDecl {
    pub identifier: String,
    pub during: String,
    pub pre_condition: Vec<ConditionDecl>,
    pub post_condition: Vec<ConditionDecl>,
}

impl ContractDecl {
    pub fn new() -> ContractDecl {
        ContractDecl {
            identifier: "".to_string(),
            during: "".to_string(),
            pre_condition: vec![],
            post_condition: vec![],
        }
    }
}

