mod declaration;
mod import;

use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    comfy,
    parser::{
        assignment::initial::VariableKeyword,
        ast::{
            identifier::Identifier,
            import::{ImportSource, ImportSpecifier},
            literal_value::LiteralValue,
            range::RangeType,
            vars::VariableDeclarator,
            ASTNode, Expression,
        },
        expression::template_literal::TemplateLiteralFragment,
        operations::{assignment::AssignmentOperator, binary::BinaryOperator},
    },
};
use hashbrown::HashMap;

pub fn interpret(program: ASTNode) -> Result<SymbolTable, String> {
    let nodes = match program {
        ASTNode::Program { body } => body,
        _ => unreachable!(),
    };

    let mut symbol_table = SymbolTable::new();

    symbol_table.interpret(nodes)?;

    println!("{:?}", symbol_table);
    Ok(symbol_table)
}

pub fn interpret_import(program: ASTNode) -> Result<SymbolTable, String> {
    let nodes = match program {
        ASTNode::Program { body } => body,
        _ => unreachable!(),
    };

    let mut symbol_table = SymbolTable::new();

    symbol_table.interpret_import(nodes)?;

    Ok(symbol_table)
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    pub functions: HashMap<String, InterpretedFn>,
    pub constants: HashMap<String, Expression>,
    pub variables: HashMap<String, Expression>,
    pub exported: HashMap<String, InterpretedFn>,
}

#[derive(Clone)]
pub struct InterpretedFn {
    pub name: String,
    pub executable: Rc<dyn Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            constants: HashMap::new(),
            variables: HashMap::new(),
            exported: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, nodes: Vec<ASTNode>) -> Result<Expression, String> {
        for node in nodes {
            match node {
                ASTNode::Program { .. } => unreachable!(),
                ASTNode::ImportDeclaration { specifiers, source } => {
                    self.import(source, specifiers)?;
                }
                ASTNode::VariableDeclaration { declarations, kind } => {
                    self.add_declarations(kind, declarations)?;
                }
                ASTNode::Assignment {
                    operator,
                    id,
                    assigned,
                } => match id {
                    Expression::MemberExpression {
                        indexed,
                        property,
                        computed,
                    } => todo!(),
                    Expression::IdentifierExpression(Identifier { name }) => {
                        let current_value = self.get_variable(&name)?;

                        if self.constants.get(&name).is_some() {
                            return Err(format!("Cannot reassign constant '{}'", name));
                        }

                        match operator {
                            AssignmentOperator::Equal => {
                                self.reassign_variable(name, assigned);
                            }
                            AssignmentOperator::PlusEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(current_value.to_owned()),
                                    operator: BinaryOperator::Plus,
                                    right: Box::new(assigned),
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::MinusEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(current_value.to_owned()),
                                    operator: BinaryOperator::Minus,
                                    right: Box::new(assigned),
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::TimesEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(current_value.to_owned()),
                                    operator: BinaryOperator::Times,
                                    right: Box::new(assigned),
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::DivideEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(current_value.to_owned()),
                                    operator: BinaryOperator::Divide,
                                    right: Box::new(assigned),
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::ModuloEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(current_value.to_owned()),
                                    operator: BinaryOperator::Modulo,
                                    right: Box::new(assigned),
                                })?;
                                self.reassign_variable(name, expr);
                            }
                        }
                    }
                    _ => unreachable!(),
                },
                ASTNode::ExpressionStatement { expression } => {
                    self.evaluate_expr(expression)?;
                }
                ASTNode::FunctionDeclaration { .. } => self.add_function(node),
                ASTNode::ForStatement {
                    declarations,
                    kind,
                    source,
                    body,
                } => todo!(),
                ASTNode::WhileStatement { test, body } => todo!(),
                ASTNode::IfStatement {
                    test,
                    body,
                    alternate,
                } => todo!(),
                ASTNode::MatchStatement { test, body } => todo!(),
                ASTNode::BlockStatement { body } => todo!(),
                ASTNode::ReturnStatement { argument, .. } => {
                    return Ok(self.evaluate_expr(argument)?)
                }
            }
        }

        Ok(Expression::Literal {
            value: LiteralValue::Nil,
            raw: "nil".to_string(),
        })
    }

    pub fn interpret_import(&mut self, nodes: Vec<ASTNode>) -> Result<Expression, String> {
        for node in nodes {
            match node {
                ASTNode::Program { .. } => unreachable!(),
                ASTNode::ImportDeclaration { specifiers, source } => {
                    self.import(source, specifiers)?;
                }
                ASTNode::VariableDeclaration { declarations, kind } => {
                    self.add_declarations(kind, declarations)?;
                }
                ASTNode::Assignment {
                    operator,
                    id,
                    assigned,
                } => match id {
                    Expression::MemberExpression {
                        indexed,
                        property,
                        computed,
                    } => todo!(),
                    Expression::IdentifierExpression(Identifier { name }) => {
                        let current_value = self.get_variable(&name)?;

                        if self.constants.get(&name).is_some() {
                            return Err(format!("Cannot reassign constant '{}' in import", name));
                        }

                        match operator {
                            AssignmentOperator::Equal => {
                                self.reassign_variable(name, assigned);
                            }
                            AssignmentOperator::PlusEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(current_value.to_owned()),
                                    operator: BinaryOperator::Plus,
                                    right: Box::new(assigned),
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::MinusEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(current_value.to_owned()),
                                    operator: BinaryOperator::Minus,
                                    right: Box::new(assigned),
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::TimesEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(current_value.to_owned()),
                                    operator: BinaryOperator::Times,
                                    right: Box::new(assigned),
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::DivideEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(current_value.to_owned()),
                                    operator: BinaryOperator::Divide,
                                    right: Box::new(assigned),
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::ModuloEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(current_value.to_owned()),
                                    operator: BinaryOperator::Modulo,
                                    right: Box::new(assigned),
                                })?;
                                self.reassign_variable(name, expr);
                            }
                        }
                    }
                    _ => unreachable!(),
                },
                ASTNode::ExpressionStatement { .. } => (),
                ASTNode::FunctionDeclaration { .. } => self.add_function(node),
                ASTNode::ForStatement {
                    declarations,
                    kind,
                    source,
                    body,
                } => todo!(),
                ASTNode::WhileStatement { test, body } => todo!(),
                ASTNode::IfStatement {
                    test,
                    body,
                    alternate,
                } => todo!(),
                ASTNode::MatchStatement { test, body } => todo!(),
                ASTNode::BlockStatement { body } => todo!(),
                ASTNode::ReturnStatement { argument, .. } => {
                    return Ok(self.evaluate_expr(argument)?)
                }
            }
        }

        Ok(Expression::Literal {
            value: LiteralValue::Nil,
            raw: "nil".to_string(),
        })
    }

    pub fn evaluate_expr(&self, expression: Expression) -> Result<Expression, String> {
        match expression {
            Expression::Literal { .. } => Ok(expression),
            Expression::Array { elements } => Ok(Expression::Array {
                elements: elements
                    .into_iter()
                    .map(|el| self.evaluate_expr(el))
                    .collect::<Result<Vec<Expression>, String>>()?,
            }),
            Expression::Object { .. } => Ok(expression),
            Expression::Range { from, to, limits } => {
                let from = if from.is_none() {
                    from
                } else {
                    Some(Box::new(self.evaluate_expr(*from.unwrap())?))
                };
                let to = if from.is_none() {
                    to
                } else {
                    Some(Box::new(self.evaluate_expr(*to.unwrap())?))
                };

                Ok(Expression::Range { from, limits, to })
            }
            Expression::FnExpression { .. } => Ok(expression),
            Expression::ErrorPropagation(expr) => {
                let expr = self.evaluate_expr(*expr)?;
                match expr {
                    Expression::Err(e) => return Err(e),
                    Expression::Ok(val) => return Ok(*val),
                    expr => Ok(expr),
                }
            }
            Expression::Err(_) => Ok(expression),
            Expression::Ok(_) => Ok(expression),

            Expression::TemplateLiteral { value, .. } => {
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

                Ok(Expression::Literal {
                    value: LiteralValue::Str(string.to_owned()),
                    raw: string,
                })
            }

            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate_expr(*left)?;
                let right = self.evaluate_expr(*right)?;

                match operator {
                    BinaryOperator::Plus => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1 + num2;

                                Ok(Expression::Literal {
                                    value: LiteralValue::Number(num),
                                    raw: num.to_string(),
                                })
                            }
                            (LiteralValue::Str(mut s1), LiteralValue::Str(s2)) => {
                                s1.push_str(&s2);

                                Ok(Expression::Literal {
                                    value: LiteralValue::Str(s1.to_owned()),
                                    raw: s1,
                                })
                            }
                            _ => return Err(format!("Cannot add {} with {}", left, right).into()),
                        },
                        (
                            Expression::Array { elements },
                            Expression::Array {
                                elements: elements_2,
                            },
                        ) => {
                            let mut array = elements;

                            array.extend(elements_2);

                            Ok(Expression::Array {
                                elements: array.to_vec(),
                            })
                        }
                        (
                            Expression::Object { properties },
                            Expression::Object {
                                properties: props_2,
                            },
                        ) => {
                            let mut props = properties;

                            props.extend(props_2);

                            Ok(Expression::Object {
                                properties: props.to_vec(),
                            })
                        }
                        _ => return Err(format!("Cannot add {} with {}", left, right).into()),
                    },
                    BinaryOperator::Minus => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1 - num2;

                                Ok(Expression::Literal {
                                    value: LiteralValue::Number(num),
                                    raw: num.to_string(),
                                })
                            }
                            _ => {
                                return Err(
                                    format!("Cannot substract {} with {}", left, right).into()
                                )
                            }
                        },
                        _ => return Err(format!("Cannot substract {} with {}", left, right).into()),
                    },
                    BinaryOperator::Times => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1 * num2;

                                Ok(Expression::Literal {
                                    value: LiteralValue::Number(num),
                                    raw: num.to_string(),
                                })
                            }
                            _ => {
                                return Err(
                                    format!("Cannot multiply {} with {}", left, right).into()
                                )
                            }
                        },
                        _ => return Err(format!("Cannot multiply {} with {}", left, right).into()),
                    },
                    BinaryOperator::Divide => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1 / num2;

                                Ok(Expression::Literal {
                                    value: LiteralValue::Number(num),
                                    raw: num.to_string(),
                                })
                            }
                            _ => {
                                return Err(format!("Cannot divide {} with {}", left, right).into())
                            }
                        },
                        _ => return Err(format!("Cannot divide {} with {}", left, right).into()),
                    },
                    BinaryOperator::Exponential => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1.powf(num2);

                                Ok(Expression::Literal {
                                    value: LiteralValue::Number(num),
                                    raw: num.to_string(),
                                })
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
                    },
                    BinaryOperator::Equal => {
                        return Ok(Expression::Literal {
                            value: LiteralValue::Boolean(left == right),
                            raw: (left == right).to_string(),
                        })
                    }
                    BinaryOperator::NotEqual => {
                        return Ok(Expression::Literal {
                            value: LiteralValue::Boolean(left != right),
                            raw: (left != right).to_string(),
                        })
                    }
                    BinaryOperator::Modulo => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                let num = num1 % num2;

                                Ok(Expression::Literal {
                                    value: LiteralValue::Number(num),
                                    raw: num.to_string(),
                                })
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
                    BinaryOperator::Greater => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                Ok(Expression::Literal {
                                    value: LiteralValue::Boolean(num1 > num2),
                                    raw: (num1 > num2).to_string(),
                                })
                            }
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
                    },
                    BinaryOperator::GreaterOrEqual => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                Ok(Expression::Literal {
                                    value: LiteralValue::Boolean(num1 >= num2),
                                    raw: (num1 >= num2).to_string(),
                                })
                            }
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
                    },
                    BinaryOperator::Smaller => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                Ok(Expression::Literal {
                                    value: LiteralValue::Boolean(num1 < num2),
                                    raw: (num1 < num2).to_string(),
                                })
                            }
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
                    },
                    BinaryOperator::SmallerOrEqual => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => match (value, value_2) {
                            (LiteralValue::Number(num1), LiteralValue::Number(num2)) => {
                                Ok(Expression::Literal {
                                    value: LiteralValue::Boolean(num1 <= num2),
                                    raw: (num1 <= num2).to_string(),
                                })
                            }
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
                    },
                    BinaryOperator::And => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => Ok(Expression::Literal {
                            value: LiteralValue::Boolean(!(value.is_falsy() || value_2.is_falsy())),
                            raw: (!(value.is_falsy() || value_2.is_falsy())).to_string(),
                        }),
                        (Expression::Array { elements }, Expression::Literal { value, .. }) => {
                            Ok(Expression::Literal {
                                value: LiteralValue::Boolean(
                                    !(elements.is_empty() || value.is_falsy()),
                                ),
                                raw: (!(elements.is_empty() || value.is_falsy())).to_string(),
                            })
                        }
                        (Expression::Literal { value, .. }, Expression::Array { elements }) => {
                            Ok(Expression::Literal {
                                value: LiteralValue::Boolean(
                                    !(elements.is_empty() || value.is_falsy()),
                                ),
                                raw: (!(elements.is_empty() || value.is_falsy())).to_string(),
                            })
                        }
                        (Expression::Array { elements }, _) => Ok(Expression::Literal {
                            value: LiteralValue::Boolean(!elements.is_empty()),
                            raw: (!elements.is_empty()).to_string(),
                        }),
                        (_, Expression::Array { elements }) => Ok(Expression::Literal {
                            value: LiteralValue::Boolean(!elements.is_empty()),
                            raw: (!elements.is_empty()).to_string(),
                        }),
                        _ => Ok(Expression::Literal {
                            value: LiteralValue::Boolean(true),
                            raw: true.to_string(),
                        }),
                    },
                    BinaryOperator::Or => match (left.to_owned(), right.to_owned()) {
                        (
                            Expression::Literal { value, .. },
                            Expression::Literal { value: value_2, .. },
                        ) => Ok(Expression::Literal {
                            value: LiteralValue::Boolean(!value.is_falsy() || !value_2.is_falsy()),
                            raw: (!value.is_falsy() || !value_2.is_falsy()).to_string(),
                        }),
                        (Expression::Array { elements }, Expression::Literal { value, .. }) => {
                            Ok(Expression::Literal {
                                value: LiteralValue::Boolean(
                                    !elements.is_empty() || !value.is_falsy(),
                                ),
                                raw: (!elements.is_empty() || !value.is_falsy()).to_string(),
                            })
                        }
                        (Expression::Literal { value, .. }, Expression::Array { elements }) => {
                            Ok(Expression::Literal {
                                value: LiteralValue::Boolean(
                                    !elements.is_empty() || !value.is_falsy(),
                                ),
                                raw: (!elements.is_empty() || !value.is_falsy()).to_string(),
                            })
                        }
                        _ => Ok(Expression::Literal {
                            value: LiteralValue::Boolean(true),
                            raw: true.to_string(),
                        }),
                    },
                }
            }

            Expression::MemberExpression {
                indexed,
                property,
                computed,
            } => {
                let evaluated_indexed = self.evaluate_expr((*indexed).to_owned())?;
                match evaluated_indexed {
                    Expression::Literal { value, .. } => {
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
                            match evaluated_property {
                                Expression::Literal { value, .. } => match value {
                                    LiteralValue::Number(num) => {
                                        let index = num as usize;
                                        if index > (str_value.len() - 1) {
                                            return Err(format!("Index out of range, tried to index at index {}, but length is {}", num.floor(), str_value.len()));
                                        }
                                        let new_str = &str_value[index..index + 1];

                                        let raw = format!("\"{}\"", new_str);
                                        return Ok(Expression::Literal {
                                            value: LiteralValue::Str(new_str.to_owned()),
                                            raw,
                                        });
                                    }
                                    _ => {
                                        return Err(format!(
                                            "Expected a `Number` or a `Range` to index {}",
                                            indexed
                                        ))
                                    }
                                },
                                Expression::Range { from, limits, to } => {
                                    let max_index = (str_value.len() - 1) as isize;

                                    let start_index = match from {
                                        Some(expr) => {
                                            match *expr {
                                                Expression::Literal { value, .. } => {
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
                                            match *expr {
                                                Expression::Literal { value, .. } => {
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

                                    return Ok(Expression::Literal {
                                        value: LiteralValue::Str(new_str.to_owned()),
                                        raw: new_str.to_owned(),
                                    });
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
                    Expression::Array { elements } => {
                        if computed {
                            let evaluated_property = self.evaluate_expr((*property).to_owned())?;
                            match evaluated_property {
                                Expression::Literal { value, .. } => match value {
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
                                Expression::Range { from, limits, to } => {
                                    let max_index = (elements.len() - 1) as isize;

                                    let start_index = match from {
                                        Some(expr) => {
                                            match *expr {
                                                Expression::Literal { value, .. } => {
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
                                                        match *expr {
                                                            Expression::Literal { value, .. } => {
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
                                                    match *expr {
                                                        Expression::Literal { value, .. } => {
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

                                    return Ok(Expression::Array {
                                        elements: new_value,
                                    });
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
            Expression::CallExpression { callee, args } => match *callee {
                Expression::MemberExpression {
                    indexed,
                    property,
                    computed,
                } => todo!(),
                Expression::CallExpression { callee, args } => todo!(),
                Expression::IdentifierExpression(Identifier { name }) => match name.as_str() {
                    "print" => Ok(comfy::print(self, args)?),
                    "input" => Ok(comfy::input(self, args)?),
                    name => {
                        let x = self.get_function(name)?;

                        Ok((x.executable)(self, args)?)
                    }
                },
                Expression::FnExpression {
                    params,
                    body,
                    is_shortcut,
                    return_type,
                } => todo!(),
                Expression::Parenthesized(expr) => {
                    let expr = self.evaluate_expr(*expr)?;
                    Ok(Expression::Literal {
                        value: LiteralValue::Nil,
                        raw: "".to_owned(),
                    })
                }
                _ => unreachable!(),
            },
            Expression::IdentifierExpression(Identifier { name }) => {
                let value = self.get_variable(&name)?;

                Ok(value.to_owned())
            }
            Expression::Parenthesized(expr) => self.evaluate_expr(*expr),
            Expression::Comment { .. } => unreachable!("Cannot evaluate a comment"),
        }
    }

    fn get_variable(&self, name: &str) -> Result<&Expression, String> {
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
    fn reassign_variable(&mut self, name: String, expr: Expression) {
        self.variables.insert(name, expr);
    }
    fn add_variable(&mut self, name: String, expr: Expression) {
        self.constants.remove(&name);
        self.variables.insert(name, expr);
    }

    fn add_constant(&mut self, name: String, expr: Expression) {
        self.variables.remove(&name);
        self.constants.insert(name, expr);
    }

    fn get_function(&self, name: &str) -> Result<&InterpretedFn, String> {
        let value = self.functions.get(name);

        if let Some(value) = value {
            return Ok(value);
        }

        Err(format!("Undefined function '{}'", name))
    }
    fn export_function(&mut self, name: &str) -> Result<InterpretedFn, String> {
        // when importing another symbol table, thus after reading, parsing and importing another file
        let value = self.exported.remove(name);

        if let Some(value) = value {
            return Ok(value);
        }

        Err(format!("No function '{}' exported", name))
    }
    fn add_function(&mut self, function: ASTNode) {
        match function {
            ASTNode::FunctionDeclaration {
                id,
                params,
                body,
                is_exported,
                ..
            } => {
                let name = id.to_owned().name;
                let symbol_table = Rc::new(RefCell::new(self.clone()));

                let executable: Rc<
                    dyn Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String>,
                > = Rc::new(
                    move |actual_symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let mut local_symbol_table = symbol_table.borrow_mut();

                        if args.len() != params.len() {
                            return Err(format!(
                                "Expected {} arguments when calling function `{}`",
                                params.len(),
                                id.to_owned().name
                            ));
                        }

                        for (i, param) in params.iter().enumerate() {
                            let value = actual_symbol_table.evaluate_expr(args[i].to_owned())?;

                            local_symbol_table.add_variable(param.id.to_owned().name, value);
                        }

                        let result = match *body.to_owned() {
                            ASTNode::BlockStatement { body } => {
                                local_symbol_table.interpret(body)?
                            }
                            ASTNode::ReturnStatement { argument, .. } => {
                                local_symbol_table.evaluate_expr(argument)?
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

                self.functions
                    .insert(name.to_owned(), InterpretedFn { name, executable });
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
