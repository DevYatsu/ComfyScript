mod declaration;
mod import;

use std::{fmt::Debug, rc::Rc};

use crate::{
    comfy::{self},
    parser::{
        assignment::initial::VariableKeyword,
        ast::{
            identifier::Identifier,
            import::{ImportSource, ImportSpecifier},
            literal_value::LiteralValue,
            range::RangeType,
            vars::VariableDeclarator,
            Expression, ExpressionKind, Program, Statement, StatementKind,
        },
        expression::template_literal::TemplateLiteralFragment,
        function::{return_expression::ReturnStatement, FunctionBody, FunctionDeclaration},
        operations::{assignment::AssignmentOperator, binary::BinaryOperator},
    },
};
use hashbrown::HashMap;

pub trait RunnableCode {
    fn get_statements(self) -> Vec<Statement>;
}

pub fn interpret(program: impl RunnableCode) -> Result<SymbolTable, String> {
    let mut symbol_table = SymbolTable::new();

    symbol_table.interpret(program)?;

    println!("{:?}", symbol_table);
    Ok(symbol_table)
}

pub fn interpret_import(program: Program) -> Result<SymbolTable, String> {
    let mut symbol_table = SymbolTable::new();

    symbol_table.interpret_import(program)?;

    Ok(symbol_table)
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    pub functions: HashMap<String, InterpretedFn>,
    pub constants: HashMap<String, Expression>,
    pub variables: HashMap<String, Expression>,
    pub exported: HashMap<String, InterpretedFn>,
    pub scopes: Vec<SymbolTable>,
}

#[derive(Clone)]
pub struct InterpretedFn {
    pub name: String,
    pub executable: Rc<dyn Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        comfy::init_std_functions(&mut functions);

        Self {
            functions,
            constants: HashMap::new(),
            variables: HashMap::new(),
            exported: HashMap::new(),
            scopes: Vec::new(),
        }
    }
    pub fn new_scope() -> Self {
        Self {
            functions: HashMap::new(),
            constants: HashMap::new(),
            variables: HashMap::new(),
            exported: HashMap::new(),
            scopes: Vec::new(),
        }
    }
    fn add_scope(&mut self) {
        self.scopes.push(SymbolTable::new_scope())
    }
    fn remove_last_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn interpret(&mut self, program: impl RunnableCode) -> Result<Expression, String> {
        for node in program.get_statements() {
            match node.kind {
                StatementKind::Import(source, specifiers) => {
                    self.import(source, specifiers)?;
                }
                StatementKind::VariableDeclaration(kind, declarations) => {
                    self.add_declarations(kind, declarations)?;
                }
                StatementKind::Assignment(id, operator, assigned) => match id.kind {
                    ExpressionKind::MemberExpression {
                        indexed,
                        property,
                        computed,
                    } => todo!(),
                    ExpressionKind::IdentifierExpression(Identifier(name)) => {
                        let current_value = self.get_variable(&name)?;

                        if self.constants.get(&name).is_some() {
                            return Err(format!("Cannot reassign constant '{}'", name));
                        }

                        match operator {
                            AssignmentOperator::Equal => {
                                self.reassign_variable(name, assigned)?;
                            }
                            AssignmentOperator::PlusEqual => {
                                let expr = self.evaluate_expr(Expression::with_kind(
                                    ExpressionKind::BinaryExpression(
                                        Box::new(current_value.to_owned()),
                                        BinaryOperator::Plus,
                                        Box::new(assigned),
                                    ),
                                ))?;
                                self.reassign_variable(name, expr)?;
                            }
                            AssignmentOperator::MinusEqual => {
                                let expr = self.evaluate_expr(Expression::with_kind(
                                    ExpressionKind::BinaryExpression(
                                        Box::new(current_value.to_owned()),
                                        BinaryOperator::Minus,
                                        Box::new(assigned),
                                    ),
                                ))?;
                                self.reassign_variable(name, expr)?;
                            }
                            AssignmentOperator::TimesEqual => {
                                let expr = self.evaluate_expr(Expression::with_kind(
                                    ExpressionKind::BinaryExpression(
                                        Box::new(current_value.to_owned()),
                                        BinaryOperator::Times,
                                        Box::new(assigned),
                                    ),
                                ))?;
                                self.reassign_variable(name, expr)?;
                            }
                            AssignmentOperator::DivideEqual => {
                                let expr = self.evaluate_expr(Expression::with_kind(
                                    ExpressionKind::BinaryExpression(
                                        Box::new(current_value.to_owned()),
                                        BinaryOperator::Divide,
                                        Box::new(assigned),
                                    ),
                                ))?;
                                self.reassign_variable(name, expr)?;
                            }
                            AssignmentOperator::ModuloEqual => {
                                let expr = self.evaluate_expr(Expression::with_kind(
                                    ExpressionKind::BinaryExpression(
                                        Box::new(current_value.to_owned()),
                                        BinaryOperator::Modulo,
                                        Box::new(assigned),
                                    ),
                                ))?;
                                self.reassign_variable(name, expr)?;
                            }
                        }
                    }
                    _ => unreachable!(),
                },
                StatementKind::Expression(expression) => {
                    self.evaluate_expr(expression)?;
                }
                StatementKind::FunctionDeclaration { .. } => self.add_function(node),
                StatementKind::ForStatement(kind, declarations, source, body) => todo!(),
                StatementKind::WhileStatement { .. } => todo!(),
                StatementKind::IfStatement { .. } => todo!(),
                StatementKind::MatchStatement(test, body) => todo!(),
                StatementKind::ReturnStatement(ReturnStatement(argument, ..)) => {
                    // maybe put ReturnStatement inside of StatementKind::Expression ?
                    return Ok(self.evaluate_expr(argument)?);
                }
            }
        }

        Ok((LiteralValue::Nil, "nil".to_string()).into())
    }

    pub fn interpret_import(&mut self, program: impl RunnableCode) -> Result<Expression, String> {
        for node in program.get_statements() {
            match node.kind {
                StatementKind::Import(source, specifiers) => {
                    self.import(source, specifiers)?;
                }
                StatementKind::VariableDeclaration(kind, declarations) => {
                    self.add_declarations(kind, declarations)?;
                }
                StatementKind::Assignment(id, operator, assigned) => match id.kind {
                    ExpressionKind::MemberExpression {
                        indexed,
                        property,
                        computed,
                    } => todo!(),
                    ExpressionKind::IdentifierExpression(Identifier(name)) => {
                        let current_value = self.get_variable(&name)?;

                        if self.constants.get(&name).is_some() {
                            return Err(format!("Cannot reassign constant '{}'", name));
                        }

                        match operator {
                            AssignmentOperator::Equal => {
                                self.reassign_variable(name, assigned)?;
                            }
                            AssignmentOperator::PlusEqual => {
                                let expr = self.evaluate_expr(Expression::with_kind(
                                    ExpressionKind::BinaryExpression(
                                        Box::new(current_value.to_owned()),
                                        BinaryOperator::Plus,
                                        Box::new(assigned),
                                    ),
                                ))?;
                                self.reassign_variable(name, expr)?;
                            }
                            AssignmentOperator::MinusEqual => {
                                let expr = self.evaluate_expr(Expression::with_kind(
                                    ExpressionKind::BinaryExpression(
                                        Box::new(current_value.to_owned()),
                                        BinaryOperator::Minus,
                                        Box::new(assigned),
                                    ),
                                ))?;
                                self.reassign_variable(name, expr)?;
                            }
                            AssignmentOperator::TimesEqual => {
                                let expr = self.evaluate_expr(Expression::with_kind(
                                    ExpressionKind::BinaryExpression(
                                        Box::new(current_value.to_owned()),
                                        BinaryOperator::Times,
                                        Box::new(assigned),
                                    ),
                                ))?;
                                self.reassign_variable(name, expr)?;
                            }
                            AssignmentOperator::DivideEqual => {
                                let expr = self.evaluate_expr(Expression::with_kind(
                                    ExpressionKind::BinaryExpression(
                                        Box::new(current_value.to_owned()),
                                        BinaryOperator::Divide,
                                        Box::new(assigned),
                                    ),
                                ))?;
                                self.reassign_variable(name, expr)?;
                            }
                            AssignmentOperator::ModuloEqual => {
                                let expr = self.evaluate_expr(Expression::with_kind(
                                    ExpressionKind::BinaryExpression(
                                        Box::new(current_value.to_owned()),
                                        BinaryOperator::Modulo,
                                        Box::new(assigned),
                                    ),
                                ))?;
                                self.reassign_variable(name, expr)?;
                            }
                        }
                    }
                    _ => unreachable!(),
                },
                StatementKind::Expression(expression) => {
                    self.evaluate_expr(expression)?;
                }
                StatementKind::FunctionDeclaration { .. } => self.add_function(node),
                StatementKind::ForStatement(kind, declarations, source, body) => todo!(),
                StatementKind::WhileStatement { .. } => todo!(),
                StatementKind::IfStatement { .. } => todo!(),
                StatementKind::MatchStatement(test, body) => todo!(),
                StatementKind::ReturnStatement(ReturnStatement(argument, ..)) => {
                    // maybe put ReturnStatement inside of StatementKind::Expression ?
                    return Ok(self.evaluate_expr(argument)?);
                }
            }
        }

        Ok((LiteralValue::Nil, "nil".to_string()).into())
    }

    pub fn evaluate_expr(&self, expression: Expression) -> Result<Expression, String> {
        match expression.kind {
            ExpressionKind::Literal(..) => Ok(expression),
            ExpressionKind::Array(elements) => Ok(Expression::with_kind(ExpressionKind::Array(
                elements
                    .into_iter()
                    .map(|el| self.evaluate_expr(el))
                    .collect::<Result<Vec<Expression>, String>>()?,
            ))),
            ExpressionKind::Object(..) => Ok(expression),
            ExpressionKind::Range(from, limits, to) => {
                let from = if from.is_none() {
                    from
                } else {
                    Some(Box::new(self.evaluate_expr(*from.unwrap())?))
                };
                let to = if to.is_none() {
                    to
                } else {
                    Some(Box::new(self.evaluate_expr(*to.unwrap())?))
                };

                Ok(Expression::with_kind(ExpressionKind::Range(
                    from, limits, to,
                )))
            }
            ExpressionKind::FnExpression { .. } => Ok(expression),
            ExpressionKind::ErrorPropagation(expr) => {
                let expr = self.evaluate_expr(*expr)?;
                match expr.kind {
                    ExpressionKind::Err(e) => return Err(e),
                    ExpressionKind::Ok(val) => return Ok(*val),
                    _ => Ok(expr),
                }
            }
            ExpressionKind::Err(_) => Ok(expression),
            ExpressionKind::Ok(_) => Ok(expression),

            ExpressionKind::TemplateLiteral(value, ..) => {
                let mut string = String::new();

                for fragment in value {
                    match fragment {
                        TemplateLiteralFragment::Literal(literal) => string.push_str(&literal),
                        TemplateLiteralFragment::EscapedChar(c) => string.push(c),
                        TemplateLiteralFragment::Expression(expr) => {
                            string.push_str(&self.evaluate_expr(expr)?.console_print())
                        }
                        TemplateLiteralFragment::EscapedWS => (),
                    }
                }

                Ok((LiteralValue::Str(string.to_owned()), string).into())
            }

            ExpressionKind::BinaryExpression(left, operator, right) => {
                let left = self.evaluate_expr(*left)?;
                let right = self.evaluate_expr(*right)?;

                match operator {
                    BinaryOperator::Plus => match (left.kind.to_owned(), right.kind.to_owned()) {
                        (
                            ExpressionKind::Literal(value, ..),
                            ExpressionKind::Literal(value_2, ..),
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1 + num2;

                                Ok((LiteralValue::Number(num), num.to_string()).into())
                            }
                            (LiteralValue::Str(mut s1), LiteralValue::Str(s2)) => {
                                s1.push_str(&s2);

                                Ok((LiteralValue::Str(s1.to_owned()), s1).into())
                            }
                            _ => return Err(format!("Cannot add {} with {}", left, right).into()),
                        },
                        (ExpressionKind::Array(elements), ExpressionKind::Array(elements_2)) => {
                            let mut array = elements;

                            array.extend(elements_2);

                            Ok(Expression::with_kind(ExpressionKind::Array(array.to_vec())))
                        }
                        (ExpressionKind::Object(properties), ExpressionKind::Object(props_2)) => {
                            let mut props = properties;

                            props.extend(props_2);

                            Ok(Expression::with_kind(ExpressionKind::Object(
                                props.to_vec(),
                            )))
                        }
                        _ => return Err(format!("Cannot add {} with {}", left, right).into()),
                    },
                    BinaryOperator::Minus => match (left.kind.to_owned(), right.kind.to_owned()) {
                        (
                            ExpressionKind::Literal(value, ..),
                            ExpressionKind::Literal(value_2, ..),
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1 - num2;

                                Ok((LiteralValue::Number(num), num.to_string()).into())
                            }
                            _ => {
                                return Err(
                                    format!("Cannot substract {} with {}", left, right).into()
                                )
                            }
                        },
                        _ => return Err(format!("Cannot substract {} with {}", left, right).into()),
                    },
                    BinaryOperator::Times => match (left.kind.to_owned(), right.kind.to_owned()) {
                        (
                            ExpressionKind::Literal(value, ..),
                            ExpressionKind::Literal(value_2, ..),
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1 * num2;

                                Ok((LiteralValue::Number(num), num.to_string()).into())
                            }
                            _ => {
                                return Err(
                                    format!("Cannot multiply {} with {}", left, right).into()
                                )
                            }
                        },
                        _ => return Err(format!("Cannot multiply {} with {}", left, right).into()),
                    },
                    BinaryOperator::Divide => match (left.kind.to_owned(), right.kind.to_owned()) {
                        (
                            ExpressionKind::Literal(value, ..),
                            ExpressionKind::Literal(value_2, ..),
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1 / num2;

                                Ok((LiteralValue::Number(num), num.to_string()).into())
                            }
                            _ => {
                                return Err(format!("Cannot divide {} with {}", left, right).into())
                            }
                        },
                        _ => return Err(format!("Cannot divide {} with {}", left, right).into()),
                    },
                    BinaryOperator::Exponential => {
                        match (left.kind.to_owned(), right.kind.to_owned()) {
                            (
                                ExpressionKind::Literal(value, ..),
                                ExpressionKind::Literal(value_2, ..),
                            ) => match (value, value_2) {
                                (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                    let num = num1.powf(num2);

                                    Ok((LiteralValue::Number(num), num.to_string()).into())
                                }
                                _ => {
                                    return Err(format!(
                                        "Cannot calculate {} raised to the power of {}",
                                        left, right
                                    )
                                    .into())
                                }
                            },
                            _ => {
                                return Err(format!(
                                    "Cannot calculate {} raised to the power of {}",
                                    left, right
                                )
                                .into())
                            }
                        }
                    }
                    BinaryOperator::Equal => {
                        return Ok((
                            LiteralValue::Boolean(left == right),
                            (left == right).to_string(),
                        )
                            .into())
                    }
                    BinaryOperator::NotEqual => {
                        return Ok((
                            LiteralValue::Boolean(left != right),
                            (left != right).to_string(),
                        )
                            .into())
                    }
                    BinaryOperator::Modulo => match (left.kind.to_owned(), right.kind.to_owned()) {
                        (
                            ExpressionKind::Literal(value, ..),
                            ExpressionKind::Literal(value_2, ..),
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1 % num2;

                                Ok((LiteralValue::Number(num), num.to_string()).into())
                            }
                            _ => {
                                return Err(
                                    format!("Cannot calculate {} modulo {}", left, right).into()
                                )
                            }
                        },
                        _ => {
                            return Err(format!("Cannot calculate {} modulo {}", left, right).into())
                        }
                    },
                    BinaryOperator::Greater => {
                        match (left.kind.to_owned(), right.kind.to_owned()) {
                            (
                                ExpressionKind::Literal(value, ..),
                                ExpressionKind::Literal(value_2, ..),
                            ) => match (value, value_2) {
                                (LiteralValue::Number(num1), LiteralValue::Number(num2)) => Ok((
                                    LiteralValue::Boolean(num1 > num2),
                                    (num1 > num2).to_string(),
                                )
                                    .into()),
                                _ => {
                                    return Err(format!(
                                        "Cannot compare {} for '>' equality {}",
                                        left, right
                                    )
                                    .into())
                                }
                            },
                            _ => {
                                return Err(format!(
                                    "Cannot compare {} for '>' equality {}",
                                    left, right
                                )
                                .into())
                            }
                        }
                    }
                    BinaryOperator::GreaterOrEqual => {
                        match (left.kind.to_owned(), right.kind.to_owned()) {
                            (
                                ExpressionKind::Literal(value, ..),
                                ExpressionKind::Literal(value_2, ..),
                            ) => match (value, value_2) {
                                (LiteralValue::Number(num1), LiteralValue::Number(num2)) => Ok((
                                    LiteralValue::Boolean(num1 >= num2),
                                    (num1 >= num2).to_string(),
                                )
                                    .into()),
                                _ => {
                                    return Err(format!(
                                        "Cannot compare {} for '>=' equality {}",
                                        left, right
                                    )
                                    .into())
                                }
                            },
                            _ => {
                                return Err(format!(
                                    "Cannot compare {} for '>=' equality {}",
                                    left, right
                                )
                                .into())
                            }
                        }
                    }
                    BinaryOperator::Smaller => {
                        match (left.kind.to_owned(), right.kind.to_owned()) {
                            (
                                ExpressionKind::Literal(value, ..),
                                ExpressionKind::Literal(value_2, ..),
                            ) => match (value, value_2) {
                                (LiteralValue::Number(num1), LiteralValue::Number(num2)) => Ok((
                                    LiteralValue::Boolean(num1 < num2),
                                    (num1 < num2).to_string(),
                                )
                                    .into()),
                                _ => {
                                    return Err(format!(
                                        "Cannot compare {} for '<' equality {}",
                                        left, right
                                    )
                                    .into())
                                }
                            },
                            _ => {
                                return Err(format!(
                                    "Cannot compare {} for '<' equality {}",
                                    left, right
                                )
                                .into())
                            }
                        }
                    }
                    BinaryOperator::SmallerOrEqual => {
                        match (left.kind.to_owned(), right.kind.to_owned()) {
                            (
                                ExpressionKind::Literal(value, ..),
                                ExpressionKind::Literal(value_2, ..),
                            ) => match (value, value_2) {
                                (LiteralValue::Number(num1), LiteralValue::Number(num2)) => Ok((
                                    LiteralValue::Boolean(num1 <= num2),
                                    (num1 <= num2).to_string(),
                                )
                                    .into()),
                                _ => {
                                    return Err(format!(
                                        "Cannot compare {} for '<=' equality {}",
                                        left, right
                                    )
                                    .into())
                                }
                            },
                            _ => {
                                return Err(format!(
                                    "Cannot compare {} for '<=' equality {}",
                                    left, right
                                )
                                .into())
                            }
                        }
                    }
                    BinaryOperator::And => match (left.kind.to_owned(), right.kind.to_owned()) {
                        (
                            ExpressionKind::Literal(value, ..),
                            ExpressionKind::Literal(value_2, ..),
                        ) => Ok((
                            LiteralValue::Boolean(!(value.is_falsy() || value_2.is_falsy())),
                            (!(value.is_falsy() || value_2.is_falsy())).to_string(),
                        )
                            .into()),
                        (ExpressionKind::Array(elements), ExpressionKind::Literal(value, ..)) => {
                            Ok((
                                LiteralValue::Boolean(!(elements.is_empty() || value.is_falsy())),
                                (!(elements.is_empty() || value.is_falsy())).to_string(),
                            )
                                .into())
                        }
                        (ExpressionKind::Literal(value, ..), ExpressionKind::Array(elements)) => {
                            Ok((
                                LiteralValue::Boolean(!(elements.is_empty() || value.is_falsy())),
                                (!(elements.is_empty() || value.is_falsy())).to_string(),
                            )
                                .into())
                        }
                        (ExpressionKind::Array(elements), _) => Ok((
                            LiteralValue::Boolean(!elements.is_empty()),
                            (!elements.is_empty()).to_string(),
                        )
                            .into()),
                        (_, ExpressionKind::Array(elements)) => Ok((
                            LiteralValue::Boolean(!elements.is_empty()),
                            (!elements.is_empty()).to_string(),
                        )
                            .into()),
                        _ => Ok((LiteralValue::Boolean(true), true.to_string()).into()),
                    },
                    BinaryOperator::Or => match (left.kind.to_owned(), right.kind.to_owned()) {
                        (
                            ExpressionKind::Literal(value, ..),
                            ExpressionKind::Literal(value_2, ..),
                        ) => Ok((
                            LiteralValue::Boolean(!value.is_falsy() || !value_2.is_falsy()),
                            (!value.is_falsy() || !value_2.is_falsy()).to_string(),
                        )
                            .into()),
                        (ExpressionKind::Array(elements), ExpressionKind::Literal(value, ..)) => {
                            Ok((
                                LiteralValue::Boolean(!elements.is_empty() || !value.is_falsy()),
                                (!elements.is_empty() || !value.is_falsy()).to_string(),
                            )
                                .into())
                        }
                        (ExpressionKind::Literal(value, ..), ExpressionKind::Array(elements)) => {
                            Ok((
                                LiteralValue::Boolean(!elements.is_empty() || !value.is_falsy()),
                                (!elements.is_empty() || !value.is_falsy()).to_string(),
                            )
                                .into())
                        }
                        _ => Ok((LiteralValue::Boolean(true), true.to_string()).into()),
                    },
                }
            }

            ExpressionKind::MemberExpression {
                indexed,
                property,
                computed,
            } => {
                let evaluated_indexed = self.evaluate_expr((*indexed).to_owned())?;
                match evaluated_indexed.kind {
                    ExpressionKind::Literal(value, ..) => {
                        let str_value = match value {
                            LiteralValue::Str(s) => s,
                            _ => {
                                return Err(format!(
                                    "Cannot index {} as it's of type {}",
                                    indexed,
                                    value.get_type()
                                ))
                            }
                        };

                        if computed {
                            let evaluated_property = self.evaluate_expr((*property).to_owned())?;
                            match evaluated_property.kind {
                                ExpressionKind::Literal(value, ..) => match value {
                                    LiteralValue::Number(num) => {
                                        let index = num as usize;
                                        if index > (str_value.len() - 1) {
                                            return Err(format!("Index out of range, tried to index at index {}, but length is {}", num.floor(), str_value.len()));
                                        }
                                        let new_str = &str_value[index..index + 1];

                                        let raw = format!("\"{}\"", new_str);
                                        return Ok(
                                            (LiteralValue::Str(new_str.to_owned()), raw).into()
                                        );
                                    }
                                    _ => {
                                        return Err(format!(
                                            "Expected a `Number` or a `Range` to index {}",
                                            indexed
                                        ))
                                    }
                                },
                                ExpressionKind::Range(from, limits, to) => {
                                    let max_index = (str_value.len() - 1) as isize;

                                    let start_index = match from {
                                        Some(expr) => {
                                            match (*expr).kind {
                                                ExpressionKind::Literal (value, ..) => {
                                                    match value {
                                                        LiteralValue::Number(num) => num as isize,
                                                        val => return Err(format!(
                                                            "Range start index expected to be of type `Number`, found {}",
                                                                val
                                                        ))
                                                        
                                                    }
                                                },
                                                _ => return Err(format!("Range start index expected to be of type `Number`, found {}", expr))
                                            } 
                                        },
                                        None => 0,
                                    };
                                    let end_index = match to {
                                        Some(expr) => {
                                            match (*expr).kind {
                                                ExpressionKind::Literal (value, ..) => {
                                                    match value {
                                                        LiteralValue::Number(num) => num as isize,
                                                        val => return Err(format!(
                                                            "Range end index expected to be of type `Number`, found {}",
                                                                val
                                                        ))
                                                        
                                                    }
                                                },
                                                _ => return Err(format!("Range end index expected to be of type `Number`, found {}", expr)) 
                                            } 
                                        },
                                        None => max_index,
                                    };

                                    if start_index < 0 || start_index > max_index {
                                        return Err(format!("Index out of range, Range start index is {}, but length is {}", start_index, str_value.len()));
                                    }

                                    let new_str = match limits {
                                        RangeType::Dot => {
                                            if end_index < 0 || end_index > (max_index + 1) {
                                                return Err(format!("Index out of range, Range end index is {}, but length is {}", end_index, str_value.len()));
                                            }

                                            &str_value[start_index as usize..end_index as usize]
                                        }
                                        RangeType::DotEqual => {
                                            if end_index < 0 || end_index > max_index {
                                                return Err(format!("Index out of range, Range end index is {}, but length is {}", end_index, str_value.len()));
                                            }

                                            &str_value[start_index as usize..=end_index as usize]
                                        }
                                    };

                                    return Ok((
                                        LiteralValue::Str(new_str.to_owned()),
                                        new_str.to_owned(),
                                    )
                                        .into());
                                }
                                _ => {
                                    return Err(format!(
                                        "Expected a `Number` or a `Range` to index {}",
                                        indexed
                                    ))
                                }
                            }
                        }
                    }
                    ExpressionKind::Array(elements) => {
                        if computed {
                            let evaluated_property = self.evaluate_expr((*property).to_owned())?;
                            match evaluated_property.kind {
                                ExpressionKind::Literal(value, ..) => match value {
                                    LiteralValue::Number(num) => {
                                        let index = num.floor() as usize;
                                        if index > (elements.len() - 1) {
                                            return Err(format!("Index out of range, tried to index at index {}, but length is {}", num.floor(), elements.len()));
                                        }
                                        let new_value = elements[num as usize].to_owned();
                                        return Ok(new_value);
                                    }
                                    _ => {
                                        return Err(format!(
                                            "Expected a `Number` or a `Range` to index {}",
                                            indexed
                                        ))
                                    }
                                },
                                ExpressionKind::Range(from, limits, to) => {
                                    let max_index = (elements.len() - 1) as isize;

                                    let start_index = match from {
                                        Some(expr) => {
                                            match (*expr).kind {
                                                ExpressionKind::Literal (value, ..) => {
                                                    match value {
                                                        LiteralValue::Number(num) => num as isize,
                                                        val => return Err(format!(
                                                            "Range start index expected to be of type `Number`, found {}",
                                                                val
                                                        ))
                                                        
                                                    }
                                                },
                                                _ => return Err(format!("Range start index expected to be of type `Number`, found {}", expr))
                                            } 
                                        },
                                        None => 0,
                                    };

                                    if start_index < 0 || start_index > max_index {
                                        return Err(format!("Index out of range, Range start index is {}, but length is {}", start_index, elements.len()));
                                    }

                                    let new_value = match limits {
                                        RangeType::Dot => {
                                            let end_index = match to {
                                                Some(expr) => {
                                                        match (*expr).kind {
                                                            ExpressionKind::Literal (value, ..) => {
                                                                match value {
                                                                    LiteralValue::Number(num) => num as isize,
                                                                    val => return Err(format!(
                                                                        "Range end index expected to be of type `Number`, found {}",
                                                                            val
                                                                    ))
                                                                    
                                                                }
                                                            },
                                                            _ => return Err(format!("Range end index expected to be of type `Number`, found {}", expr)) 
                                                        } 
                                                    },
                                                None => max_index + 1,
                                            };

                                            if end_index < 0 || end_index > (max_index + 1) {
                                                return Err(format!("Index out of range, Range end index is {}, but length is {}", end_index, elements.len()));
                                            }

                                            elements[start_index as usize..end_index as usize]
                                                .to_vec()
                                        }
                                        RangeType::DotEqual => {
                                            let end_index = match to {
                                                Some(expr) => {
                                                    match (*expr).kind {
                                                        ExpressionKind::Literal (value, ..) => {
                                                            match value {
                                                                LiteralValue::Number(num) => num as isize,
                                                                val => return Err(format!(
                                                                    "Range end index expected to be of type `Number`, found {}",
                                                                        val
                                                                ))
                                                                
                                                            }
                                                        },
                                                        _ => return Err(format!("Range end index expected to be of type `Number`, found {}", expr)) 
                                                    } 
                                                },
                                                None => max_index,
                                            };

                                            if end_index < 0 || end_index > max_index {
                                                return Err(format!("Index out of range, Range end index is {}, but length is {}", end_index, elements.len()));
                                            }

                                            elements[start_index as usize..=end_index as usize]
                                                .to_vec()
                                        }
                                    };

                                    return Ok(Expression::with_kind(ExpressionKind::Array(
                                        new_value,
                                    )));
                                }
                                _ => {
                                    return Err(format!(
                                        "Expected a `Number` or a `Range` to index {}",
                                        indexed
                                    ))
                                }
                            }
                        }
                    }
                    ExpressionKind::Object(properties) => {}
                    ExpressionKind::Range(from, limits, to) => {}
                    ExpressionKind::Err(_) => {}
                    ExpressionKind::Ok(_) => {}
                    x => {
                        return Err(format!(
                            "Cannot index {} as it's of type {}",
                            indexed,
                            x.get_type()
                        ))
                    }
                };
                todo!()
            }
            ExpressionKind::CallExpression { callee, args } => match (*callee).kind {
                ExpressionKind::MemberExpression {
                    indexed,
                    property,
                    computed,
                } => todo!(),
                ExpressionKind::IdentifierExpression(Identifier(name)) => match name.as_str() {
                    name => {
                        let x = self.get_function(name)?;

                        Ok((x.executable)(self, args)?)
                    }
                },
                ExpressionKind::Parenthesized(expr) => {
                    let expr = self.evaluate_expr(*expr)?;

                    match expr.kind {
                        ExpressionKind::FnExpression {
                            params,
                            body,
                            is_shortcut,
                            return_type,
                        } => {
                            todo!()
                        }
                        expr => {
                            return Err(format!("Cannot call `{}`. It is not a function.", expr))
                        }
                    };

                    return Ok((LiteralValue::Nil, "".to_owned()).into());
                }
                _ => unreachable!(),
            },
            ExpressionKind::IdentifierExpression(Identifier(name)) => {
                let value = self.get_variable(&name)?;

                Ok(value.to_owned())
            }
            ExpressionKind::Parenthesized(expr) => self.evaluate_expr(*expr),
            ExpressionKind::Comment { .. } => unreachable!("Cannot evaluate a comment"),
        }
    }

    fn get_scope_variable(&self, name: &str) -> Result<&Expression, String> {
        let value = self.constants.get(name);

        if let Some(value) = value {
            return Ok(value);
        }

        let value = self.variables.get(name);

        if let Some(value) = value {
            return Ok(value);
        }

        Err(format!("Undefined variable '{}'", name))
    }
    fn get_variable(&self, name: &str) -> Result<&Expression, String> {
        for symbol_table in self.scopes.iter().rev() {
            if let Ok(value) = symbol_table.get_scope_variable(name) {
                return Ok(value);
            }
        }

        self.get_scope_variable(name)
    }

    fn reassign_variable(&mut self, name: String, expr: Expression) -> Result<(), String> {
        for symbol_table in self.scopes.iter_mut().rev() {
            if symbol_table.variables.contains_key(&name) {
                symbol_table.variables.insert(name, expr);
                return Ok(());
            }
            if symbol_table.constants.contains_key(&name) {
                return Err(format!("Cannot reassign constant `{}`", name));
            }
        }
        self.variables.insert(name, expr);

        Ok(())
    }
    fn add_variable(&mut self, name: String, expr: Expression) {
        if let Some(symbol_table) = self.scopes.last_mut() {
            symbol_table.constants.remove(&name);
            symbol_table.variables.insert(name, expr);
            return;
        }
        self.constants.remove(&name);
        self.variables.insert(name, expr);
    }

    fn add_constant(&mut self, name: String, expr: Expression) {
        if let Some(symbol_table) = self.scopes.last_mut() {
            symbol_table.variables.remove(&name);
            symbol_table.constants.insert(name, expr);
            return;
        }

        self.variables.remove(&name);
        self.constants.insert(name, expr);
    }

    fn add_scoped_function(&mut self, name: String, func: InterpretedFn) {
        if let Some(symbol_table) = self.scopes.last_mut() {
            symbol_table.functions.insert(name, func);
            return;
        }

        self.functions.insert(name, func);
    }
    fn get_function(&self, name: &str) -> Result<&InterpretedFn, String> {
        for symbol_table in self.scopes.iter().rev() {
            if let Some(value) = symbol_table.functions.get(name) {
                return Ok(value);
            }
        }

        if let Some(value) = self.functions.get(name) {
            return Ok(value);
        }

        Err(format!("Undefined function '{}'", name))
    }
    fn export_function(&mut self, name: &str) -> Result<InterpretedFn, String> {
        // when importing another symbol table, thus after reading, parsing and importing another file
        if let Some(value) = self.exported.remove(name) {
            return Ok(value);
        }

        Err(format!("No function '{}' exported", name))
    }
    fn add_function(&mut self, function: Statement) {
        match function.kind {
            StatementKind::FunctionDeclaration(FunctionDeclaration {
                id,
                params,
                body,
                is_exported,
                ..
            }) => {
                let name = id.to_owned().value();
                let symbol_table = self.to_owned();

                let executable: Rc<
                    dyn Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String>,
                > = Rc::new(
                    move |actual_symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let mut local_symbol_table = symbol_table.to_owned();

                        if args.len() != params.len() {
                            return Err(format!(
                                "Expected {} arguments when calling function `{}`",
                                params.len(),
                                id.to_owned().value()
                            ));
                        }

                        for (i, param) in params.iter().enumerate() {
                            let value = actual_symbol_table.evaluate_expr(args[i].to_owned())?;

                            local_symbol_table.add_variable(param.id.to_owned().value(), value);
                        }

                        let result = match &body {
                            FunctionBody::Block(body) => {
                                local_symbol_table.interpret(body.to_owned())?
                            }
                            FunctionBody::ShortCut(ReturnStatement(argument, ..)) => {
                                local_symbol_table.evaluate_expr(argument.clone())?
                            }
                            _ => unreachable!(),
                        };

                        Ok(result)
                    },
                );

                if is_exported {
                    self.exported.insert(
                        name.to_owned(),
                        InterpretedFn {
                            name: name.to_owned(),
                            executable: executable.to_owned(),
                        },
                    );
                }

                self.add_scoped_function(name.to_owned(), InterpretedFn { name, executable });
            }
            _ => unreachable!(),
        }
    }

    fn import(
        &mut self,
        source: ImportSource,
        specifiers: Vec<ImportSpecifier>,
    ) -> Result<(), String> {
        import::import(self, source, specifiers)
    }
    fn add_declarations(
        &mut self,
        kind: VariableKeyword,
        declarations: Vec<VariableDeclarator>,
    ) -> Result<(), String> {
        declaration::add_declarations(self, kind, declarations)
    }
}

impl Debug for InterpretedFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InterpretedFn")
            .field("name", &self.name)
            .finish()
    }
}
impl PartialEq for InterpretedFn {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
