use crate::{expected, expected_valid};

use super::{
    ast::{
        identifier::{parse_identifier, Identifier},
        import::{ImportSource, ImportSpecifier},
        literal_value::LiteralValue,
        ASTNode, Expression,
    },
    errors::expected_space,
    expression::strings::parse_string,
};
use nom::{
    character::complete::{char, multispace0, multispace1},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_import(i: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (i, _) = tag("import")(i)?;
    let (i, _) = multispace1(i)?;

    let (i, asterisk) = opt(char('*'))(i)?;

    let (i, specifiers) = if asterisk.is_none() {
        let (i, specifiers) = separated_list1(
            delimited(multispace0, char(','), multispace0),
            parse_import_specifier,
        )(i)?;

        let (i, _) = opt(preceded(multispace0, char(',')))(i)?;
        let (i, _) = multispace1.context(expected_space()).cut().parse(i)?;

        (i, specifiers)
    } else {
        let (i, local) = opt(opt_import_as)(i)?;
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

    let (i, _) = tag("from")
        .context(expected!("'from' keyword"))
        .cut()
        .parse(i)?;
    let (i, _) = multispace1.context(expected_space()).cut().parse(i)?;

    let (i, source) = parse_string
        .context(expected_valid!("import source"))
        .cut()
        .parse(i)?;

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
    let (i, imported_name) = parse_identifier.parse(i)?;
    let (i, local_name) = opt(opt_import_as)(i)?;

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

fn opt_import_as(i: &str) -> IResult<&str, Identifier, ErrorTree<&str>> {
    let (i, _) = multispace1(i)?;
    let (i, _) = tag("as")(i)?;

    let (i, _) = multispace1
        .context(expected!("a local import identifer"))
        .cut()
        .parse(i)?;
    let (i, local_name) = parse_identifier
        .context(expected!("a local import identifer"))
        .cut()
        .parse(i)?;

    return Ok((i, local_name));
}
