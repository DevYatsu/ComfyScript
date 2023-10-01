pub mod identifier;
pub mod import;
pub mod literal_value;
mod object;
pub mod vars;

use self::{
    identifier::Identifier, import::ImportSpecifier, literal_value::LiteralValue, object::Property,
    vars::VariableDeclarator,
};

use super::{
    assignment::initial::VariableKeyword, operations::{assignment::AssignmentOperator, binary::BinaryOperator},
};

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program {
        body: Vec<ASTNode>,
        source_type: String, // use this for modules or main file
    },
    ImportDeclaration {
        specifiers: Vec<ImportSpecifier>,
        source: Expression,
    },

    VariableDeclaration {
        declarations: Vec<VariableDeclarator>,
        kind: VariableKeyword,
    },
    ExpressionStatement {
        expression: Expression,
    }, // everything that is not a real statement, that is for example strings and numbers or var reassigment

    FunctionDeclaration {
        id: Identifier,
        params: Vec<Identifier>,
        body: Vec<ASTNode>,
    },
    ForStatement {
        declarations: Vec<VariableDeclarator>,
        source: Expression,
        body: Vec<ASTNode>,
    },
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
        name: String,
    },
    BinaryExpression {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    MemberExpression {
        object: Identifier,
        property: Identifier,
        computed: bool,
        optional: bool,
    },
    CallExpression {
        callee: Box<Expression>,
        args: Vec<Expression>,
        optional: bool,
    },
    AssignmentExpression {
        operator: AssignmentOperator,
        id: Identifier,
        assigned: Box<Expression>,
    },
    IdentifierExpression {
        identifier: Identifier,
    },
}
