use crate::expected_valid;

use super::{
    ast::{
        identifier::{parse_identifier, Identifier},
        import::{ImportSource, ImportSpecifier},
        literal_value::LiteralValue,
        ASTNode, Expression,
    },
    expression::strings::parse_string, errors::{expected_expression, expected_space},
};
use nom::{
    character::complete::{multispace0, multispace1},
    combinator::opt,
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_import(i: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (i, _) = tag("import")(i)?;
    let (i, _) = multispace1(i)?;

    let (i, asterisk) = opt(tag("*"))(i)?;

    let (i, specifiers) = if asterisk.is_none() {
        let (i, specifiers) = separated_list1(tag(","), parse_import_specifier)(i)?;
        let (i, _) = multispace0(i)?;
        let (i, comma) = opt(tag(","))(i)?;

        if comma.is_some() {
            let (i, _) = multispace1.context(expected_space()).parse(i)?;
            (i, specifiers)
        } else {
            (i, specifiers)
        }
    } else {
        let (i, local) = opt_import_as(i)?;
        let (i, _) = multispace0(i)?;

        let asterisk = Identifier {
            name: asterisk.unwrap().to_string(),
        };

        if let Some(local) = local {
            (
                i,
                vec![ImportSpecifier {
                    local,
                    imported: asterisk,
                }],
            )
        } else {
            (
                i,
                vec![ImportSpecifier {
                    local: asterisk.to_owned(),
                    imported: asterisk,
                }],
            )
        }
    };

    let (i, _) = tag("from").context(expected_valid!("import source")).parse(i)?;
    let (i, _) = multispace1.context(expected_space()).parse(i)?;

    let (i, source) = parse_string.context(expected_valid!("import source")).parse(i)?; 

    let source = match source {
        Expression::Literal { value, .. } => match value {
            LiteralValue::Str(value) => ImportSource { value },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Ok((i, ASTNode::ImportDeclaration { specifiers, source }))
}

fn parse_import_specifier(i: &str) -> IResult<&str, ImportSpecifier, ErrorTree<&str>> {
    let (i, _) = multispace0(i)?;
    let (i, imported_name) = parse_identifier
        .context(expected_valid!("import identifier"))
        .parse(i)?;
    let (i, local_name) = opt_import_as(i)?;
    let (i, _) = multispace0(i)?;

    match local_name {
        Some(local_name) => {
            return Ok((
                i,
                ImportSpecifier {
                    local: local_name,
                    imported: imported_name,
                },
            ));
        }
        None => Ok((
            i,
            ImportSpecifier {
                local: imported_name.to_owned(),
                imported: imported_name,
            },
        )),
    }
}

fn opt_import_as(i: &str) -> IResult<&str, Option<Identifier>, ErrorTree<&str>> {
    let (i, opt_val) = opt(preceded(multispace1, tag("as")))(i)?;

    if opt_val.is_some() {
        let (i, _) = multispace1.context(expected_space()).parse(i)?;
        let (i, local_name) = parse_identifier
            .context(expected_valid!("import identifer"))
            .parse(i)?;

        return Ok((i, Some(local_name)));
    }
    let (i, _) = multispace0(i)?;

    Ok((i, None))
}
