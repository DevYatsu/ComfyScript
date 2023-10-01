use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::opt,
    error::VerboseError,
    multi::separated_list1,
    IResult,
};

use crate::parser::ast::identifier::parse_identifier;

use super::{
    assignment::initial::VariableKeyword,
    ast::{vars::VariableDeclarator, ASTNode},
    parse_input, Span,
};

pub fn parse_for_statement(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, _) = tag("for")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, opt_keyword) = opt(alt((tag("let"), tag("var"))))(input)?;

    let keyword = if let Some(k) = opt_keyword {
        match k.fragment() {
            &"let" => VariableKeyword::Let,
            _ => VariableKeyword::Var,
        }
    } else {
        VariableKeyword::Var
    };

    let (input, identifiers) = separated_list1(tag(","), parse_identifier)(input)?;
    let (input, _) = multispace1(input)?;

    let (input, _) = tag("in")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, indexed) = parse_identifier(input)?; // todo!! call a fn to parse expression here
    let (input, _) = multispace0(input)?; // for now we parse an identifier

    let (input, _) = tag("{")(input)?;

    // parse for content
    // todo!!

    let (input, body) = parse_input(input, Some("}"))?;

    Ok((
        input,
        ASTNode::ForStatement {
            declarations: identifiers
                .into_iter()
                .map(|id| VariableDeclarator {
                    id,
                    init: super::ast::Expression::Array { elements: vec![] },
                })
                .collect(),
            source: todo!(),
            body,
        },
    ))
}
