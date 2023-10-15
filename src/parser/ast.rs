pub mod identifier;
pub mod import;
pub mod literal_value;
pub mod object;
pub mod range;
pub mod vars;

use self::{
    identifier::Identifier,
    import::{ImportSource, ImportSpecifier},
    literal_value::LiteralValue,
    object::Property,
    range::RangeType,
    vars::VariableDeclarator,
};
use super::{
    assignment::initial::VariableKeyword,
    operations::{assignment::AssignmentOperator, binary::BinaryOperator},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program {
        body: Vec<ASTNode>,
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
        id: Option<Identifier>,
        // if None then anon func
        params: Vec<Identifier>,
        body: Box<ASTNode>,

        is_shortcut: bool,
        // if is_shortcut == true then body = ASTNode::ReturnStatement
    },
    ForStatement {
        declarations: Vec<Identifier>,
        kind: VariableKeyword,
        source: Expression,
        body: Box<ASTNode>,
    },
    WhileStatement {
        test: Expression,
        body: Box<ASTNode>,
    },
    IfStatement {
        test: Expression,
        body: Box<ASTNode>,
        alternate: Option<Box<ASTNode>>,
        // alternate may either be None, a BlockStatement or an IfStatement
    },
    BlockStatement {
        body: Vec<ASTNode>,
    },
    ReturnStatement {
        argument: Expression,
        is_shortcut: bool,
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
    Range {
        // similar to rust for instance 0..10
        from: Box<Expression>,
        limits: RangeType,
        to: Box<Expression>,
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
        // either an array indexing or an object indexing
        indexed: Box<Expression>,
        property: Box<Expression>,
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
    Comment {
        is_line: bool,
        raw_value: String,
    },
    Method {
        params: Vec<Identifier>,
        body: Box<ASTNode>,
        is_shortcut: bool,
    },
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
            ASTNode::FunctionDeclaration {
                id, params, body, ..
            } => {
                let is_anonymous = id.is_some();

                if is_anonymous {
                    write!(f, "anon fn(")?;
                } else {
                    write!(f, "fn {}(", id.clone().unwrap())?;
                }

                for param in params {
                    write!(f, "{},", param)?;
                }

                write!(f, ")")?;

                write!(f, " {}", body)
                // either put a block statement or a return statement (with shortcut)
            }
            ASTNode::ForStatement {
                declarations,
                source,
                body,
                kind,
            } => {
                write!(f, "for {kind} ")?;

                for declaration in declarations {
                    write!(f, "{},", declaration)?;
                }
                write!(f, " in ")?;
                write!(f, "{}", source)?;

                write!(f, " {}", body)
            }
            ASTNode::WhileStatement { test, body } => {
                write!(f, "while ")?;

                write!(f, "{test}")?;

                write!(f, " {}", body)
            }
            ASTNode::ReturnStatement {
                argument,
                is_shortcut,
            } => {
                if *is_shortcut {
                    write!(f, ">> ")?;
                } else {
                    write!(f, "return ")?;
                }

                write!(f, "{}", argument)?;

                write!(f, ";")
            }
            ASTNode::IfStatement {
                test,
                body,
                alternate,
            } => {
                write!(f, "if {test} {body}")?;

                if let Some(alternate) = alternate {
                    write!(f, " else ")?;
                    write!(f, "{alternate}")
                } else {
                    write!(f, "")
                }
            }
            ASTNode::BlockStatement { body } => {
                write!(f, " {{")?;
                for node in body {
                    write!(f, "{}", node)?;
                }
                write!(f, "}}")
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
                indexed, property, ..
            } => {
                write!(f, "{}.{};", indexed, property)
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
            Expression::Comment { raw_value, .. } => {
                write!(f, "{}", raw_value)
            }
            Expression::Method { params, body, .. } => {
                write!(f, "anon fn(")?;

                for param in params {
                    write!(f, "{},", param)?;
                }

                write!(f, ")")?;

                write!(f, " {}", body)
            }
            Expression::Range { from, limits, to } => {
                write!(f, "{from}{limits}{to}")
            }
        }
    }
}

impl Into<Expression> for ASTNode {
    fn into(self) -> Expression {
        match self {
            ASTNode::FunctionDeclaration {
                params,
                body,
                is_shortcut,
                ..
            } => Expression::Method {
                params,
                body,
                is_shortcut,
            },
            _ => todo!(),
        }
    }
}
