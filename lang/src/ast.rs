
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    LetDeclaration {
        name: Option<String>,
        value: Box<AST>,
        line: usize, // for error msgs
    },

    IfStatement {
        condition: Box<AST>,
        body: Vec<AST>,
        line: usize,
    },

    Import {
        file: Option<String>,
        as_: Option<String>,
        line: usize,
    },

    Object {
        properties: HashMap<String, AST>,
        line: usize,
    },

    PropertyAccess {
        object: Option<String>,
        property: Option<String>,
        line: usize,
    },

    PropertyCall {
        object: Option<String>,
        property: Option<String>,
        args: Vec<AST>,
        line: usize,
    },

    Call {
        name: String,
        args: Vec<AST>,
        line: usize,
    },

    Function {
        name: String,
        args: Vec<String>,
        body: Vec<AST>,
        line: usize,
    },

    Return {
        value: Box<AST>,
        line: usize,
    },

    InternalFunction {
        name: String,
        args: Vec<String>,
        call_fn: fn(Vec<AST>, &mut HashMap<String, AST>) -> Result<AST, String>,
    },

    Exists {
        value: Box<AST>,
        line: usize,
    },

    IsEqual {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    IsUnequal {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    Addition {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    Subtraction {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    Identifer(String),

    Number(i64),

    String(String),

    Boolean(bool),

    Float(f64),

    Null,

    Semicolon,
}


impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // TODO: Implement
            AST::String(s) => write!(f, "{}", s),
            AST::Number(n) => write!(f, "{}", n),
            AST::Float(n) => write!(f, "{}", n),
            AST::Boolean(b) => write!(f, "{}", b),
            AST::Null => write!(f, "null"),
            _ => write!(f, "{:?}", self),
        }
    }
}