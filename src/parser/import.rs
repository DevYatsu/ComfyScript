use super::{
    ast::{
        identifier::{parse_identifier, Identifier},
        import::{ImportSource, ImportSpecifier},
        literal_value::LiteralValue,
        ASTNode, Expression,
    },
    expression::strings::parse_string,
    Span,
};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::opt,
    multi::separated_list1,
    IResult,
};
use nom_supreme::error::ErrorTree;

pub fn parse_import(i: Span) -> IResult<Span, ASTNode, ErrorTree<Span>> {
    let (i, _) = tag("import")(i)?;
    let (i, _) = multispace1(i)?;

    let (i, asterisk) = opt(tag("*"))(i)?;

    let (i, specifiers) = if asterisk.is_none() {
        let (i, specifiers) = separated_list1(tag(","), parse_import_specifier)(i)?;
        let (i, _) = multispace0(i)?;
        let (i, comma) = opt(tag(","))(i)?;

        if comma.is_some() {
            let (i, _) = multispace1(i)?;
            (i, specifiers)
        } else {
            (i, specifiers)
        }
    } else {
        let (i, _) = multispace1(i)?;
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

    let (i, _) = tag("from")(i)?;
    let (i, _) = multispace1(i)?;

    let (i, source) = parse_string(i)?; // todo! add expression Literal support instead

    let source = match source {
        Expression::Literal { value, .. } => match value {
            LiteralValue::Str(value) => ImportSource { value },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Ok((i, ASTNode::ImportDeclaration { specifiers, source }))
}

fn parse_import_specifier(i: Span) -> IResult<Span, ImportSpecifier, ErrorTree<Span>> {
    let (i, _) = multispace0(i)?;
    let (i, imported_name) = parse_identifier(i)?;
    let (i, _) = multispace0(i)?;
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

fn opt_import_as(i: Span) -> IResult<Span, Option<Identifier>, ErrorTree<Span>> {
    let (i, opt_val) = opt(tag("as"))(i)?;

    if opt_val.is_some() {
        let (i, _) = multispace1(i)?;
        let (i, local_name) = parse_identifier(i)?;

        return Ok((i, Some(local_name)));
    }

    //todo!! add support for "as" keyword to rename import locally

    Ok((i, None))
}
