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
    expression::template_literal::TemplateLiteralFragment,
    function::{FunctionParam, FunctionReturnType},
    match_block::MatchBlock,
    operations::{assignment::AssignmentOperator, binary::BinaryOperator},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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
    Assignment {
        operator: AssignmentOperator,
        id: Expression,
        assigned: Expression,
    },

    ExpressionStatement {
        expression: Expression,
    }, // everything that is not a real statement, that is for example strings and numbers or var reassigment

    FunctionDeclaration {
        id: Identifier,
        // if None then anon func
        params: Vec<FunctionParam>,
        body: Box<ASTNode>,
        return_type: Option<FunctionReturnType>,
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
    MatchStatement {
        test: Expression,
        body: MatchBlock,
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
        // value/expression/expression syntax
        value: Vec<TemplateLiteralFragment>,
        raw: String,
        // syntax like this: #"hey {name}, I am {age} years old"
        // here first value is ""
    },
    Range {
        // similar to rust for instance 0..10
        from: Option<Box<Expression>>,
        limits: RangeType,
        to: Option<Box<Expression>>,
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
        // can be an IdentifierExpression or a MemberExpression  depending if it's a function call or a FnExpression call
        args: Vec<Expression>,
    },
    IdentifierExpression(Identifier),
    Parenthesized(Box<Expression>),
    Comment {
        is_line: bool,
        raw_value: String,
    },
    FnExpression {
        params: Vec<FunctionParam>,
        body: Box<ASTNode>,
        is_shortcut: bool,
        return_type: Option<FunctionReturnType>,
    },

    FallibleExpression(Box<Expression>), // An expression that can fail
}

// display is used to minify the content
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

                write!(f, " from {};", source)
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
            ASTNode::Assignment {
                operator,
                id,
                assigned,
            } => {
                write!(f, "{}", id)?;
                write!(f, "{}", operator)?;
                write!(f, "{}", assigned)
            }
            ASTNode::ExpressionStatement { expression } => {
                write!(f, "{};", expression)
            }
            ASTNode::FunctionDeclaration {
                id,
                params,
                body,
                return_type,
                ..
            } => {
                write!(f, "fn {}(", id.clone())?;

                for (i, param) in params.into_iter().enumerate() {
                    if i == params.len() - 1 {
                        write!(f, "{}", param)?;
                    } else {
                        write!(f, "{},", param)?;
                    }
                }

                write!(f, ")")?;

                if let Some(return_type) = return_type {
                    write!(f, "{}", return_type)?;
                }

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

                write!(f, "{}", argument)
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
            ASTNode::MatchStatement { test, body } => {
                write!(f, "match {test} {body}")
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
            Expression::TemplateLiteral { raw, .. } => {
                write!(f, "{}", raw)
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
                write!(f, "{}.{}", indexed, property)
            }
            Expression::CallExpression { callee, args } => {
                write!(f, "{}(", callee)?;
                for (i, arg) in args.into_iter().enumerate() {
                    if i == args.len() - 1 {
                        write!(f, "{}", arg)?;
                    } else {
                        write!(f, "{},", arg)?;
                    }
                }

                write!(f, ")")
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
            Expression::FnExpression {
                params,
                body,
                return_type,
                ..
            } => {
                write!(f, "|")?;

                for (i, param) in params.into_iter().enumerate() {
                    if i == params.len() - 1 {
                        write!(f, "{}", param)?;
                    } else {
                        write!(f, "{},", param)?;
                    }
                }

                write!(f, "|")?;

                if let Some(return_type) = return_type {
                    write!(f, "{}", return_type)?;
                }

                write!(f, " {}", body)
            }
            Expression::Range { from, limits, to } => {
                if let Some(from) = from {
                    write!(f, "{from}")?;
                }

                write!(f, "{limits}")?;

                if let Some(to) = to {
                    write!(f, "{to}")?;
                }

                write!(f, "")
            }
            Expression::FallibleExpression(expr) => {
                write!(f, "{}?", expr)
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
                return_type,
                ..
            } => Expression::FnExpression {
                params,
                body,
                is_shortcut,
                return_type,
            },
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal { value: l_value, .. }, Self::Literal { value: r_value, .. }) => {
                l_value == r_value
            }
            (
                Self::TemplateLiteral { value: l_value, .. },
                Self::TemplateLiteral { value: r_value, .. },
            ) => l_value == r_value,
            (
                Self::Range {
                    from: l_from,
                    limits: l_limits,
                    to: l_to,
                },
                Self::Range {
                    from: r_from,
                    limits: r_limits,
                    to: r_to,
                },
            ) => {
                let l_from = if l_from.is_none() {
                    Some(Box::new(Expression::Literal {
                        value: LiteralValue::Number(0.0),
                        raw: "0".to_string(),
                    }))
                } else {
                    l_from.to_owned()
                };
                let r_from = if r_from.is_none() {
                    Some(Box::new(Expression::Literal {
                        value: LiteralValue::Number(0.0),
                        raw: "0".to_string(),
                    }))
                } else {
                    r_from.to_owned()
                };

                l_from == r_from && l_limits == r_limits && l_to == r_to
            }
            (
                Self::Array {
                    elements: l_elements,
                },
                Self::Array {
                    elements: r_elements,
                },
            ) => l_elements == r_elements,
            (
                Self::Object {
                    properties: l_properties,
                },
                Self::Object {
                    properties: r_properties,
                },
            ) => l_properties == r_properties,
            (
                Self::BinaryExpression {
                    left: l_left,
                    operator: l_operator,
                    right: l_right,
                },
                Self::BinaryExpression {
                    left: r_left,
                    operator: r_operator,
                    right: r_right,
                },
            ) => l_left == r_left && l_operator == r_operator && l_right == r_right,
            (
                Self::MemberExpression {
                    indexed: l_indexed,
                    property: l_property,
                    computed: l_computed,
                },
                Self::MemberExpression {
                    indexed: r_indexed,
                    property: r_property,
                    computed: r_computed,
                },
            ) => l_indexed == r_indexed && l_property == r_property && l_computed == r_computed,
            (
                Self::CallExpression {
                    callee: l_callee,
                    args: l_args,
                },
                Self::CallExpression {
                    callee: r_callee,
                    args: r_args,
                },
            ) => l_callee == r_callee && l_args == r_args,
            (Self::IdentifierExpression(l0), Self::IdentifierExpression(r0)) => l0 == r0,
            (Self::Parenthesized(l0), Self::Parenthesized(r0)) => l0 == r0,
            (
                Self::Comment {
                    is_line: l_is_line,
                    raw_value: l_raw_value,
                },
                Self::Comment {
                    is_line: r_is_line,
                    raw_value: r_raw_value,
                },
            ) => l_is_line == r_is_line && l_raw_value == r_raw_value,
            (
                Self::FnExpression {
                    params: l_params,
                    body: l_body,
                    is_shortcut: l_is_shortcut,
                    return_type: l_return_type,
                },
                Self::FnExpression {
                    params: r_params,
                    body: r_body,
                    is_shortcut: r_is_shortcut,
                    return_type: r_return_type,
                },
            ) => {
                l_params == r_params
                    && l_body == r_body
                    && l_is_shortcut == r_is_shortcut
                    && l_return_type == r_return_type
            }
            (Self::FallibleExpression(l0), Self::FallibleExpression(r0)) => l0 == r0,
            _ => false,
        }
    }
}
