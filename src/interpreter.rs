use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    comfy,
    parser::{
        assignment::initial::VariableKeyword,
        ast::{identifier::Identifier, literal_value::LiteralValue, ASTNode, Expression},
        expression,
        operations::{assignment::AssignmentOperator, binary::BinaryOperator},
    },
};
use hashbrown::HashMap;

pub fn interpret(program: ASTNode) -> Result<(), String> {
    let nodes = match program {
        ASTNode::Program { body } => body,
        _ => unreachable!(),
    };

    let mut symbol_table = SymbolTable::new();

    symbol_table.interpret(nodes)?;

    println!("{:?}", symbol_table);
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTable {
    pub functions: HashMap<String, InterpretedFn>,
    pub constants: HashMap<String, Expression>,
    pub variables: HashMap<String, Expression>,
}

#[derive(Clone)]
pub struct InterpretedFn {
    pub node: ASTNode,
    pub executable: Rc<dyn Fn(Vec<Expression>) -> Result<Expression, String>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            constants: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, nodes: Vec<ASTNode>) -> Result<Expression, String> {
        for node in nodes {
            match node {
                ASTNode::Program { body } => todo!(),
                ASTNode::ImportDeclaration { specifiers, source } => todo!(),
                ASTNode::VariableDeclaration { declarations, kind } => {
                    // let target_table = match kind {
                    //     VariableKeyword::Var => &mut self.variables,
                    //     VariableKeyword::Let => &mut self.constants,
                    // };

                    // need to update this code to only match kind once
                    for declaration in declarations {
                        let name = declaration.id.name.to_owned();
                        let expr = self.evaluate_expr(declaration.init)?;

                        match kind {
                            VariableKeyword::Var => self.variables.insert(name, expr),
                            VariableKeyword::Let => self.constants.insert(name, expr),
                        };
                    }
                }
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

    pub fn evaluate_expr(&mut self, expression: Expression) -> Result<Expression, String> {
        match expression {
            Expression::Literal { .. } => Ok(expression),
            Expression::Array { elements } => Ok(Expression::Array {
                elements: elements
                    .into_iter()
                    .map(|el| self.evaluate_expr(el))
                    .collect::<Result<Vec<Expression>, String>>()?,
            }),
            Expression::Object { .. } => Ok(expression),
            Expression::Range { .. } => Ok(expression),
            Expression::FallibleExpression(expr) => Ok(self.evaluate_expr(*expr)?),
            Expression::FnExpression { .. } => Ok(expression),

            Expression::TemplateLiteral { value, .. } => {
                let mut string = String::new();

                for fragment in value {
                    match fragment {
                        expression::template_literal::TemplateLiteralFragment::Literal(literal) => {
                            string.push_str(&literal)
                        }
                        expression::template_literal::TemplateLiteralFragment::EscapedChar(c) => {
                            string.push(c)
                        }
                        expression::template_literal::TemplateLiteralFragment::Expression(expr) => {
                            string.push_str(&self.evaluate_expr(expr)?.to_string())
                        }
                        expression::template_literal::TemplateLiteralFragment::EscapedWS => (),
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
            } => todo!(),
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

                        Ok((x.executable)(args)?)
                    }
                },
                Expression::FnExpression {
                    params,
                    body,
                    is_shortcut,
                    return_type,
                } => todo!(),
                Expression::FallibleExpression(_) => todo!(),
                Expression::Parenthesized(expr) => {
                    let expr = self.evaluate_expr(*expr)?;
                    Ok(Expression::Literal {
                        value: LiteralValue::Nil,
                        raw: "".to_owned(),
                    })
                }
                _ => unreachable!(),
            },
            Expression::AssignmentExpression {
                operator,
                id,
                assigned,
            } => {
                match *id {
                    Expression::MemberExpression {
                        indexed,
                        property,
                        computed,
                    } => todo!(),
                    Expression::CallExpression { callee, args } => todo!(),
                    Expression::IdentifierExpression(Identifier { name }) => {
                        let var = self.get_variable(&name)?;

                        if self.constants.get(&name).is_some() {
                            return Err(format!("Cannot reassign constant '{}'", name));
                        }

                        match operator {
                            AssignmentOperator::Equal => {
                                self.reassign_variable(name, *assigned);
                            }
                            AssignmentOperator::PlusEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(var.to_owned()),
                                    operator: BinaryOperator::Plus,
                                    right: assigned,
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::MinusEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(var.to_owned()),
                                    operator: BinaryOperator::Minus,
                                    right: assigned,
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::TimesEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(var.to_owned()),
                                    operator: BinaryOperator::Times,
                                    right: assigned,
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::DivideEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(var.to_owned()),
                                    operator: BinaryOperator::Divide,
                                    right: assigned,
                                })?;
                                self.reassign_variable(name, expr);
                            }
                            AssignmentOperator::ModuloEqual => {
                                let expr = self.evaluate_expr(Expression::BinaryExpression {
                                    left: Box::new(var.to_owned()),
                                    operator: BinaryOperator::Modulo,
                                    right: assigned,
                                })?;
                                self.reassign_variable(name, expr);
                            }
                        }
                    }
                    _ => unreachable!(),
                }

                Ok(Expression::Literal {
                    value: LiteralValue::Nil,
                    raw: "".to_owned(),
                })
            }
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
    fn get_function(&self, name: &str) -> Result<&InterpretedFn, String> {
        let value = self.functions.get(name);

        if let Some(value) = value {
            return Ok(value);
        }

        Err(format!("Undefined function '{}'", name))
    }
    fn reassign_variable(&mut self, name: String, expr: Expression) {
        self.variables.insert(name, expr);
    }

    fn add_function(&mut self, function: ASTNode) {
        let node = function.clone();

        match function {
            ASTNode::FunctionDeclaration {
                id, params, body, ..
            } => {
                let name = id.to_owned().name;
                let symbol_table = Rc::new(RefCell::new(self.clone()));

                let executable: Rc<dyn Fn(Vec<Expression>) -> Result<Expression, String>> =
                    Rc::new(move |args: Vec<Expression>| -> Result<Expression, String> {
                        let mut local_symbol_table = symbol_table.borrow_mut();

                        if args.len() != params.len() {
                            return Err(format!(
                                "Expected {} arguments when calling function `{}`",
                                params.len(),
                                id.to_owned().name
                            ));
                        }

                        for (i, param) in params.iter().enumerate() {
                            local_symbol_table
                                .variables
                                .insert(param.id.to_owned().name, args[i].to_owned());
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
                    });

                self.functions
                    .insert(name, InterpretedFn { node, executable });
            }
            _ => unreachable!(),
        }
    }
}

impl Debug for InterpretedFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InterpretedFn")
            .field("node", &self.node)
            .finish()
    }
}
impl PartialEq for InterpretedFn {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}