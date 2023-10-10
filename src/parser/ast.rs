pub mod identifier;
pub mod import;
pub mod literal_value;
pub mod object;
pub mod vars;

use std::fmt;

use self::{
    identifier::Identifier,
    import::{ImportSource, ImportSpecifier},
    literal_value::LiteralValue,
    object::Property,
    vars::VariableDeclarator,
};

use super::{
    assignment::initial::VariableKeyword,
    operations::{assignment::AssignmentOperator, binary::BinaryOperator},
};

#[derive(Debug, Clone)]
pub enum ProgramSrc {
    Module,
    Main,
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program {
        body: Vec<ASTNode>,
        source_type: ProgramSrc, // use this for modules or main file
    },
    ImportDeclaration {
        specifiers: Vec<ImportSpecifier>,
        source: ImportSource,
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
    ReturnStatement {
        argument: Expression,
        shortcut: bool,
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
        // syntax like this: #"hey {name}, I am {age} years old"
    },
    Array {
        elements: Vec<Expression>,
    },
    Object {
        properties: Vec<Property>,
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
    },
    CallExpression {
        callee: Box<Expression>,
        // can be an IdentifierExpression or a MemberExpression  depending if it's a function call or a method call
        args: Vec<Expression>,
    },
    AssignmentExpression {
        operator: AssignmentOperator,
        id: Identifier,
        assigned: Box<Expression>,
    },
    IdentifierExpression(Identifier),
    Parenthesized(Box<Expression>),
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Program { body, .. } => Ok(for node in body {
                write!(f, "{}", node)?;
            }),
            ASTNode::ImportDeclaration { specifiers, source } => {
                write!(f, "import ")?;

                for (i, specifier) in specifiers.iter().enumerate() {
                    if i == specifiers.len() - 1 {
                        write!(f, "{}", specifier)?;
                    } else {
                        write!(f, "{},", specifier)?;
                    }
                }

                write!(f, " from \"{}\";", source)
            }
            ASTNode::VariableDeclaration { declarations, kind } => {
                write!(f, "{} ", kind)?;

                for (i, declaration) in declarations.iter().enumerate() {
                    if i == declarations.len() - 1 {
                        write!(f, "{}", declaration)?;
                    } else {
                        write!(f, "{},", declaration)?;
                    }
                }

                write!(f, ";")
            }
            ASTNode::ExpressionStatement { expression } => {
                write!(f, "{};", expression)
            }
            ASTNode::FunctionDeclaration { id, params, body } => {
                write!(f, "fn {}(", id)?;
                for param in params {
                    write!(f, "{},", param)?;
                }
                write!(f, "){{")?;
                for node in body {
                    write!(f, "{}", node)?;
                }

                write!(f, "}}")
            }
            ASTNode::ForStatement {
                declarations,
                source,
                body,
            } => {
                write!(f, "for ")?;

                for declaration in declarations {
                    write!(f, "{},", declaration)?;
                }
                write!(f, " in ")?;
                write!(f, "{}", source)?;

                write!(f, " {{")?;
                for node in body {
                    write!(f, "{}", node)?;
                }

                write!(f, "}}")
            }
            ASTNode::WhileStatement { test, body } => {
                write!(f, "while ")?;

                write!(f, "{test}")?;

                write!(f, " {{")?;
                for node in body {
                    write!(f, "{}", node)?;
                }

                write!(f, "}}")
            }
            ASTNode::ReturnStatement { argument, shortcut } => {
                if *shortcut {
                    write!(f, ">> ")?;
                } else {
                    write!(f, "return ")?;
                }

                write!(f, "{}", argument)?;

                write!(f, ";")
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal { raw, .. } => {
                write!(f, "{}", raw)
            }
            Expression::TemplateLiteral { value, .. } => {
                write!(f, "{}", value)? //todo! update n the future
                ;
                todo!()
            }
            Expression::Array { elements } => {
                write!(f, "[")?;
                for element in elements {
                    write!(f, "{},", element)?;
                }

                write!(f, "]")
            }
            Expression::Object { properties } => {
                write!(f, "{{")?;
                for prop in properties {
                    write!(f, "{},", prop)?;
                }

                write!(f, "}}")
            }
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => {
                write!(f, "{}", left)?;
                write!(f, "{}", operator)?;
                write!(f, "{}", right)
            }
            Expression::MemberExpression {
                object, property, ..
            } => {
                write!(f, "{}.{};", object, property)
            }
            Expression::CallExpression { callee, args } => {
                write!(f, "{}(", callee)?;
                for arg in args {
                    write!(f, "{},", arg)?;
                }
                write!(f, ")")?;
                todo!()
            }
            Expression::AssignmentExpression {
                operator,
                id,
                assigned,
            } => {
                write!(f, "{}", id)?;
                write!(f, "{}", operator)?;
                write!(f, "{}", assigned)
            }
            Expression::IdentifierExpression(identifier) => {
                write!(f, "{}", identifier)
            }
            Expression::Parenthesized(expr) => {
                write!(f, "({})", expr)
            }
        }
    }
}
