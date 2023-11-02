pub mod identifier;
pub mod import;
pub mod literal_value;
pub mod object;
pub mod range;
pub mod vars;

use crate::interpreter::RunnableCode;

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
    data_type::DataType,
    expression::template_literal::TemplateLiteralFragment,
    function::{
        return_expression::ReturnStatement, FunctionBody, FunctionDeclaration, FunctionParam,
        ReturnType,
    },
    if_block::IfStatement,
    match_block::MatchBlock,
    operations::{assignment::AssignmentOperator, binary::BinaryOperator},
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub body: Vec<Statement>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    Import(ImportSource, Vec<ImportSpecifier>),

    /// VariableKeyword: Let or Var
    /// Vec<VariableDeclarator>: each declaration in the statement
    /// we can create several varibles at once in ComfyScript
    VariableDeclaration(VariableKeyword, Vec<VariableDeclarator>),

    /// Expression: Indexing expression or IdentifierExpression
    /// AssignmentOperator: =,+=,-=,*=,%=
    /// Expression: value assigned to the variable
    Assignment(Expression, AssignmentOperator, Expression),

    /// ExpressionStatement: may be a function call or anything that has no importance in the code
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),

    /// VariableKeyword: Set to Var initially but can be add before the variables defined in the statement
    /// Vec<Identifier>: All varibles defined in the for statement
    /// Expression: what is indexed
    /// BlockStatement: the block that is run
    ForStatement(VariableKeyword, Vec<Identifier>, Expression, BlockStatement),

    /// Expression: the test => when true then run the block
    /// BlockStatement: the block that is run
    WhileStatement(Expression, BlockStatement),

    IfStatement(IfStatement),

    /// Expression: what is matched
    /// MatchBlock: Contains the MatchCases
    MatchStatement(Expression, MatchBlock),

    ReturnStatement(ReturnStatement),
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    /// The LiteralValue contains the value of the Literal
    /// The String is the RAW representation of the Literal
    Literal(LiteralValue, String),

    /// syntax like this: #"hey {name}, I am {age} years old"
    /// The Vec contains the part of the template literal
    /// The String is the RAW representation of the template literal
    TemplateLiteral(Vec<TemplateLiteralFragment>, String),

    /// start_index: if None then equals 0
    /// Rangetype: .. or ..=
    /// end_index: if None then equals max length of what is indexed
    Range(Option<Box<Expression>>, RangeType, Option<Box<Expression>>),

    Array(Vec<Expression>),
    Object(Vec<Property>),

    /// left expression
    /// BinaryOperator
    /// right expression
    BinaryExpression(Box<Expression>, BinaryOperator, Box<Expression>),

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
        body: Box<FunctionBody>,
        is_shortcut: bool,
        return_type: Option<ReturnType>,
    },

    /// An expression propaging the Result if it fails
    /// For instance, a function call that can fail returns an Ok(expression) or a Err(String)
    /// Using the ErrorPropagation operator (?) will either return the error and stop the program or transform the Ok(expression) into expression
    ErrorPropagation(Box<Expression>),

    /// An expression that has failed
    /// Can be matched with a match statement
    Err(String),

    /// An expression that has succeed
    /// Can be matched with a match statement    
    Ok(Box<Expression>),
}

impl Statement {
    pub fn with_kind(kind: StatementKind) -> Self {
        Statement { kind }
    }
}
impl RunnableCode for Program {
    fn get_statements(self) -> Vec<Statement> {
        self.body
    }
}
impl RunnableCode for BlockStatement {
    fn get_statements(self) -> Vec<Statement> {
        self.body
    }
}
impl Expression {
    pub fn with_kind(kind: ExpressionKind) -> Self {
        Expression { kind }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let node_str: String = self
            .body
            .iter()
            .map(|node| {
                println!("{}", node);
                format!("{}", node.kind)
            })
            .collect::<Vec<String>>()
            .join(";");

        write!(f, "{}", node_str)
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, " {{")?;
        for node in &self.body {
            write!(f, "{}", node)?;
        }
        write!(f, "}}")
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Display for StatementKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            StatementKind::Assignment(id, operator, assigned) => {
                write!(f, "{}", id)?;
                write!(f, "{}", operator)?;
                write!(f, "{}", assigned)
            }
            StatementKind::Import(source, specifiers) => {
                write!(f, "import ")?;

                for (i, specifier) in specifiers.iter().enumerate() {
                    if i == specifiers.len() - 1 {
                        write!(f, "{}", specifier)?;
                    } else {
                        write!(f, "{},", specifier)?;
                    }
                }

                write!(f, " from {}", source)
            }

            StatementKind::VariableDeclaration(kind, declarations) => {
                write!(f, "{} ", kind)?;

                Ok(for (i, declaration) in declarations.iter().enumerate() {
                    if i == declarations.len() - 1 {
                        write!(f, "{}", declaration)?;
                    } else {
                        write!(f, "{},", declaration)?;
                    }
                })
            }
            StatementKind::Expression(expression) => {
                write!(f, "{};", expression)
            }

            StatementKind::FunctionDeclaration(function) => {
                write!(f, "{}", function)
            }
            StatementKind::ForStatement(kind, declarations, source, body) => {
                write!(f, "for {kind} ")?;

                for declaration in declarations {
                    write!(f, "{},", declaration)?;
                }
                write!(f, " in ")?;
                write!(f, "{}", source)?;

                write!(f, " {}", body)
            }
            StatementKind::WhileStatement(test, body) => {
                write!(f, "while {test}{body}")
            }
            StatementKind::ReturnStatement(return_statement) => {
                write!(f, "{}", return_statement)
            }
            StatementKind::IfStatement(if_statement) => {
                write!(f, "{if_statement}")
            }
            StatementKind::MatchStatement(test, body) => {
                write!(f, "match {test} {body}")
            }
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for ExpressionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionKind::Literal(_, raw) => {
                write!(f, "{}", raw)
            }
            ExpressionKind::TemplateLiteral(_, raw) => {
                write!(f, "{}", raw)
            }
            ExpressionKind::Array(elements) => {
                write!(f, "[")?;
                for element in elements {
                    write!(f, "{},", element)?;
                }

                write!(f, "]")
            }
            ExpressionKind::Object(properties) => {
                write!(f, "{{")?;
                for prop in properties {
                    write!(f, "{},", prop)?;
                }

                write!(f, "}}")
            }
            ExpressionKind::BinaryExpression(left, operator, right) => {
                write!(f, "{}", left)?;
                write!(f, "{}", operator)?;
                write!(f, "{}", right)
            }
            ExpressionKind::MemberExpression {
                indexed, property, ..
            } => {
                write!(f, "{}.{}", indexed, property)
            }
            ExpressionKind::CallExpression { callee, args } => {
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
            ExpressionKind::IdentifierExpression(identifier) => {
                write!(f, "{}", identifier)
            }
            ExpressionKind::Parenthesized(expr) => {
                write!(f, "({})", expr)
            }
            ExpressionKind::Comment { raw_value, .. } => {
                write!(f, "{}", raw_value)
            }
            ExpressionKind::FnExpression {
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
            ExpressionKind::Range(from, limits, to) => {
                if let Some(from) = from {
                    write!(f, "{from}")?;
                }

                write!(f, "{limits}")?;

                Ok(if let Some(to) = to {
                    write!(f, "{to}")?;
                })
            }
            ExpressionKind::ErrorPropagation(expr) => {
                write!(f, "{}?", expr)
            }
            ExpressionKind::Err(s) => {
                write!(f, "Err(\"{}\")", s)
            }
            ExpressionKind::Ok(expr) => {
                write!(f, "Ok({})", expr)
            }
        }
    }
}

impl PartialEq for ExpressionKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l_value, _), Self::Literal(r_value, _)) => l_value == r_value,
            (Self::TemplateLiteral(l_value, _), Self::TemplateLiteral(r_value, _)) => {
                l_value == r_value
            }

            (Self::Range(l_from, l_limits, l_to), Self::Range(r_from, r_limits, r_to)) => {
                let l_from = if l_from.is_none() {
                    Some(Box::new(Expression {
                        kind: ExpressionKind::Literal(LiteralValue::Number(0.0), "0".to_string()),
                    }))
                } else {
                    l_from.to_owned()
                };
                let r_from = if r_from.is_none() {
                    Some(Box::new(Expression {
                        kind: ExpressionKind::Literal(LiteralValue::Number(0.0), "0".to_string()),
                    }))
                } else {
                    r_from.to_owned()
                };

                l_from == r_from && l_limits == r_limits && l_to == r_to
            }
            (Self::Array(l_elements), Self::Array(r_elements)) => l_elements == r_elements,
            (Self::Object(l_properties), Self::Object(r_properties)) => {
                l_properties == r_properties
            }
            (
                Self::BinaryExpression(l_left, l_operator, l_right),
                Self::BinaryExpression(r_left, r_operator, r_right),
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

            (Self::ErrorPropagation(r0), Self::ErrorPropagation(r1)) => r0 == r1,
            (Self::Err(r0), Self::Err(r1)) => r0 == r1,
            (Self::Ok(r0), Self::Ok(r1)) => r0 == r1,
            _ => false,
        }
    }
}

impl Expression {
    pub fn console_print(&self) -> String {
        self.kind.console_print()
    }
    pub fn get_type(self) -> DataType {
        self.kind.get_type()
    }

    pub fn is_truthy(&self) -> bool {
        match &self.kind {
            ExpressionKind::Literal(val, _) => !val.is_falsy(),
            ExpressionKind::Range(_, _, _)
            | ExpressionKind::Array(_)
            | ExpressionKind::Object(_)
            | ExpressionKind::FnExpression { .. }
            | ExpressionKind::ErrorPropagation(_)
            | ExpressionKind::Err(_)
            | ExpressionKind::Ok(_) => true,
            _ => unreachable!(),
        }
    }

    pub fn ok(expr: Expression) -> Self {
        Expression::with_kind(ExpressionKind::Ok(Box::new(expr)))
    }
    pub fn err(s: String) -> Self {
        Expression::with_kind(ExpressionKind::Err(s))
    }
    pub fn err_propagation(expr: Expression) -> Self {
        Expression::with_kind(ExpressionKind::ErrorPropagation(Box::new(expr)))
    }
    pub fn parenthesized(expr: Expression) -> Self {
        Expression::with_kind(ExpressionKind::Parenthesized(Box::new(expr)))
    }
    pub fn nil() -> Self {
        Expression::with_kind(ExpressionKind::Literal(LiteralValue::Nil, "nil".to_owned()))
    }
}

impl ExpressionKind {
    pub fn console_print(&self) -> String {
        match self {
            ExpressionKind::Literal(value, _) => value.to_string(),
            _ => self.to_string(),
        }
    }
    pub fn get_type(self) -> DataType {
        match self {
            ExpressionKind::Literal(value, ..) => value.get_type(),
            ExpressionKind::TemplateLiteral(..) => DataType::String,
            ExpressionKind::Range(..) => DataType::Range,
            ExpressionKind::Array(..) => DataType::Array,
            ExpressionKind::Object(..) => DataType::Object,
            ExpressionKind::BinaryExpression(..) => {
                unreachable!("Cannot know the type of binary expression before interpretation [in ExpressionKind::get_type()]")
            }
            ExpressionKind::MemberExpression { .. } => {
                unreachable!("Cannot know the type of member expression before interpretation [in ExpressionKind::get_type()]")
            }
            ExpressionKind::CallExpression { .. } => {
                unreachable!("Cannot know the type of call expression before interpretation [in ExpressionKind::get_type()]")
            }
            ExpressionKind::IdentifierExpression(_) => {
                unreachable!("Cannot know the type of identifier expression before interpretation [in ExpressionKind::get_type()]")
            }
            ExpressionKind::Parenthesized(_) => unreachable!(
                "Cannot know the type of parenthesized expression before interpretation [in ExpressionKind::get_type()]"
            ),
            ExpressionKind::Comment { .. } => unreachable!("Comment has no data type [in ExpressionKind::get_type()]"),
            ExpressionKind::FnExpression { .. } => DataType::Fn,
            ExpressionKind::ErrorPropagation(x) => DataType::Fallible(x),
            ExpressionKind::Err(x) => DataType::Err(x),
            ExpressionKind::Ok(ok) => DataType::Ok(ok),
        }
    }
}

impl Into<Expression> for f32 {
    fn into(self) -> Expression {
        Expression::with_kind(ExpressionKind::Literal(
            LiteralValue::Number(self),
            self.to_string(),
        ))
    }
}
impl Into<f32> for Expression {
    fn into(self) -> f32 {
        match self.kind {
            ExpressionKind::Literal(value, _) => match value {
                LiteralValue::Number(x) => x,
                _ => unreachable!(
                    "Cannot transform i32 into something Other Than ExpressionKind::Literal::Number"
                ),
            },
            _ => unreachable!(
                "Cannot transform i32 into something Other Than ExpressionKind::Literal::Number"
            ),
        }
    }
}
impl Into<Expression> for bool {
    fn into(self) -> Expression {
        Expression::with_kind(ExpressionKind::Literal(
            LiteralValue::Boolean(self),
            self.to_string(),
        ))
    }
}
impl Into<bool> for Expression {
    fn into(self) -> bool {
        match self.kind {
            ExpressionKind::Literal(value, _) => match value {
                LiteralValue::Boolean(b) => b,
                _ => unreachable!(
                    "Cannot transform bool into something Other Than ExpressionKind::Literal::Boolean"
                ),
            },
            _ => unreachable!(
                "Cannot transform bool into something Other Than ExpressionKind::Literal::Boolean"
            ),
        }
    }
}
impl Into<Expression> for String {
    fn into(self) -> Expression {
        Expression::with_kind(ExpressionKind::Literal(
            LiteralValue::Str(self.to_owned()),
            self,
        ))
    }
}
impl Into<String> for Expression {
    fn into(self) -> String {
        match self.kind {
            ExpressionKind::Literal(value, _) => match value {
                LiteralValue::Str(s) => s,
                _ => unreachable!(
                    "Cannot transform String into something Other Than ExpressionKind::Literal::Str"
                ),
            },
            _ => unreachable!(
                "Cannot transform String into something Other Than ExpressionKind::Literal::Str"
            ),
        }
    }
}

impl Into<Statement> for StatementKind {
    fn into(self) -> Statement {
        Statement { kind: self }
    }
}
impl Into<Statement> for Expression {
    fn into(self) -> Statement {
        Statement {
            kind: StatementKind::Expression(self),
        }
    }
}
