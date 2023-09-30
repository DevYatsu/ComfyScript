use crate::parser::operations::Operator;
pub mod identifier;
pub mod import;
pub mod literal_value;
mod object;
pub mod vars;

use self::{
    identifier::Identifier, import::ImportSpecifier, literal_value::LiteralValue, object::Property,
    vars::VariableDeclarator,
};

use super::assignment::initial::VariableKeyword;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program {
        body: Vec<ASTNode>,
        source_type: String, // use this for modules or main file
        start: usize,
        end: usize,
    },
    ImportDeclaration {
        specifiers: Vec<ImportSpecifier>,
        source: Expression,
        start: usize,
        end: usize,
    },

    VariableDeclaration {
        declarations: Vec<VariableDeclarator>,
        kind: VariableKeyword,
        start: usize,
        end: usize,
    },
    ExpressionStatement {
        expression: Expression,
        start: usize,
        end: usize,
    }, // everything that is not a real statement, that is for example strings and numbers or var reassigment

    FunctionDeclaration {
        id: Identifier,
        params: Vec<Identifier>,
        body: Vec<ASTNode>,
        start: usize,
        end: usize,
    },
    ForStatement {},
    WhileStatement {
        test: Expression,
        body: Vec<ASTNode>,
        start: usize,
        end: usize,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal {
        value: LiteralValue, // can be either a string or a number
        raw: String,
        start: usize,
        end: usize,
    },
    TemplateLiteral {
        value: String,
        expressions: Vec<Expression>,
        start: usize,
        end: usize,
    },
    Array {
        elements: Vec<Expression>,
        start: usize,
        end: usize,
    },
    Object {
        properties: Vec<Property>,
        name: String,
        start: usize,
        end: usize,
    },
    BinaryExpression {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
        start: usize,
        end: usize,
    },
    MemberExpression {
        object: Identifier,
        property: Identifier,
        computed: bool,
        optional: bool,
        start: usize,
        end: usize,
    },
    CallExpression {
        callee: Box<Expression>,
        args: Vec<Expression>,
        optional: bool,
        start: usize,
        end: usize,
    },
}
