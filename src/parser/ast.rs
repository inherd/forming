use std::{cmp, fmt};

#[derive(Copy, Clone)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl Location {
    pub fn new(row: usize, column: usize) -> Location {
        Location {
            row,
            column
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn none() -> Location {
        Location {
            row: 0,
            column: 0,
        }
    }

    pub fn is_none(&self) -> bool {
        self.row == 0 && self.column == 0
    }
}

impl cmp::PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        (self.row == other.row && self.column == other.column) || self.is_none() || other.is_none()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} column {}", self.row, self.column)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_none() {
            write!(fmt, "{}:{}", self.row, self.column)
        } else {
            write!(fmt, "…")
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    pub node: T,
    pub location: Location
}

impl<T> Node<T> {
    pub fn new(node: T, location: Location) -> Node<T> {
        Node {
            node,
            location
        }
    }
}

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

// refs: https://github.com/vickenty/lang-c/blob/master/grammar.rustpeg
// refs: https://github.com/vickenty/lang-c
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
    input: Vec<Parameter>,
    output: Vec<Parameter>,
    pre_condition: String,
    post_condition: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    field: String,
    specifier: TypeSpecifier,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contract {
    name: String,
    during: String,
    pre_condition: String,
    post_condition: String,
}
