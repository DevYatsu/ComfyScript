use crate::parser::operations::Operator;
pub mod identifier;
pub mod literal_value;
mod object;
pub mod vars;

use self::{literal_value::LiteralValue, object::Property, vars::VariableDeclarator};

use super::assignment::initial::VariableKeyword;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program {
        body: Vec<ASTNode>,
        source_type: String, // use this for modules or main file
    },

    VariableDeclaration {
        declarations: Vec<VariableDeclarator>,
        kind: VariableKeyword,
    },
    ExpressionStatement {
        expression: Expression,
    }, // everything that is not a real statement, that is for example strings and numbers or var reassigment

    FunctionDeclaration {},
    ForStatement {},
    WhileStatement {
        test: Expression,
        body: Vec<ASTNode>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal {
        value: LiteralValue, // can be either a string or a number
        raw: String,
    },
    TemplateLiteral {
        value: String,
        expressions: Vec<Expression>,
    },
    Array {
        elements: Vec<Expression>,
    },
    Object {
        properties: Vec<Property>,
    },
    BinaryExpression {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}
