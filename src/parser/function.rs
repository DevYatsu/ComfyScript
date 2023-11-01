pub mod return_expression;

use std::fmt::Display;

use self::return_expression::{parse_return_statement, ReturnStatement};

use super::{
    ast::{
        identifier::Identifier, BlockStatement, Expression, ExpressionKind, Statement,
        StatementKind,
    },
    data_type::{parse_data_type, parse_opt_type_assignement, DataType},
    parse_block,
    reserved::DEFINED_FUNCTIONS,
};
use crate::parser::ast::identifier::parse_identifier;
use nom::{
    character::complete::{char as parse_char, multispace0, multispace1},
    multi::separated_list0,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub id: Identifier,
    pub is_exported: bool,
    pub params: Vec<FunctionParam>,
    pub body: FunctionBody,
    pub return_type: Option<ReturnType>,
    pub is_shortcut: bool,
    // if is_shortcut == true then body = ASTNode::ReturnStatement
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionBody {
    Block(BlockStatement),
    ShortCut(ReturnStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParam {
    pub id: Identifier,
    pub param_type: Option<DataType>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnType {
    pub return_type: DataType,
    pub is_fallible: bool, // may fail or not
}

pub fn parse_function(
    is_exported: bool,
) -> impl Fn(&str) -> IResult<&str, Statement, ErrorTree<&str>> {
    move |input| {
        let (input, _) = multispace1(input)?;

        let (input, id) = parse_identifier
            .verify(|id| !DEFINED_FUNCTIONS.contains(&id.0.as_str()))
            .context("invalid function name")
            .cut()
            .parse(input)?;

        let (input, _) = multispace0(input)?;
        let (input, _) = parse_char('(')
            .cut()
            .context("open parenthesis")
            .parse(input)?;
        let (input, _) = multispace0(input)?;

        let (input, params) = parse_fn_params(input)?;

        let (input, _) = multispace0(input)?;
        let (input, _) = parse_char(')').cut().parse(input)?;
        let (input, _) = multispace0(input)?;

        let (input, return_type) = parse_fn_return_type
            .terminated(multispace0)
            .opt()
            .parse(input)?;

        let (input, (body, is_shortcut)) = parse_fn_body.cut().parse(input)?;

        let node = Statement::with_kind(StatementKind::FunctionDeclaration(FunctionDeclaration {
            id,
            params,
            body,
            is_shortcut,
            return_type,
            is_exported,
        }));

        Ok((input, node))
    }
}

pub fn parse_fn_expression(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (input, _) = tag("|")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, params) = parse_fn_params(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = tag("|").cut().parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, return_type) = parse_fn_return_type
        .terminated(multispace0)
        .opt()
        .parse(input)?;

    let (input, (body, is_shortcut)) = parse_fn_body.map(|(b, s)| (Box::new(b), s)).parse(input)?;

    let node = Expression::with_kind(ExpressionKind::FnExpression {
        params,
        body,
        is_shortcut,
        return_type,
    });

    Ok((input, node))
}

fn parse_fn_params(input: &str) -> IResult<&str, Vec<FunctionParam>, ErrorTree<&str>> {
    let (input, params) = separated_list0(tag(","), parse_fn_param).parse(input)?;

    Ok((input, params))
}

fn parse_fn_body(input: &str) -> IResult<&str, (FunctionBody, bool), ErrorTree<&str>> {
    let (input, return_statement) = parse_return_statement.opt().parse(input)?;

    if let Some(return_statement) = return_statement {
        return Ok((input, (return_statement.into(), true)));
    }

    let (input, body) = parse_block.cut().parse(input)?;

    Ok((input, (body.into(), false)))
}

fn parse_fn_param(input: &str) -> IResult<&str, FunctionParam, ErrorTree<&str>> {
    let (input, id) = parse_identifier.preceded_by(multispace0).parse(input)?;
    let (input, param_type) = parse_opt_type_assignement(input)?;

    Ok((input, FunctionParam { id, param_type }))
}

fn parse_fn_return_type(input: &str) -> IResult<&str, ReturnType, ErrorTree<&str>> {
    let (input, _) = tag("->").complete().preceded_by(multispace0).parse(input)?;
    let (input, return_type) = parse_data_type
        .cut()
        .context("valid data type")
        .preceded_by(multispace0)
        .parse(input)?;

    let (input, is_fallible) = parse_char('?')
        .preceded_by(multispace0)
        .opt()
        .map(|x| x.is_some())
        .parse(input)?;

    Ok((
        input,
        ReturnType {
            return_type,
            is_fallible,
        },
    ))
}

impl Display for FunctionParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)?;

        if let Some(param_type) = &self.param_type {
            write!(f, ":{}", param_type)?;
        }

        write!(f, "")
    }
}
impl Display for ReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.return_type)?;

        if self.is_fallible {
            write!(f, "?")?;
        }

        write!(f, "")
    }
}

impl Display for FunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_exported {
            write!(f, "export ")?;
        }

        write!(f, "fn {}(", self.id)?;

        for (i, param) in (&self.params).into_iter().enumerate() {
            if i == self.params.len() - 1 {
                write!(f, "{}", param)?;
            } else {
                write!(f, "{},", param)?;
            }
        }

        write!(f, ")")?;

        if let Some(return_type) = &self.return_type {
            write!(f, "{}", return_type)?;
        }

        write!(f, " {}", self.body)
        // either put a block statement or a return statement (with shortcut)
    }
}
impl Display for FunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Into<FunctionBody> for ReturnStatement {
    fn into(self) -> FunctionBody {
        FunctionBody::ShortCut(self)
    }
}

impl Into<FunctionBody> for BlockStatement {
    fn into(self) -> FunctionBody {
        FunctionBody::Block(self)
    }
}
