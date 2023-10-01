use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::opt,
    error::VerboseError,
    multi::separated_list1,
    IResult,
};

use super::{
    ast::{
        identifier::parse_identifier,
        import::{ImportSource, ImportSpecifier},
        literal_value::LiteralValue,
        ASTNode, Expression,
    },
    primitive_values::strings::parse_string,
    Span,
};

pub fn parse_import(i: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (i, _) = tag("import")(i)?;
    let (i, _) = multispace1(i)?;

    let (i, all) = opt(tag("*"))(i)?;

    let (i, specifiers) = if None == all {
        separated_list1(tag(","), parse_import_specifier)(i)?
    } else {
        let (i, _) = multispace1(i)?;

        (i, vec![]) // no specifier means everything (*)
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

fn parse_import_specifier(i: Span) -> IResult<Span, ImportSpecifier, VerboseError<Span>> {
    let (i, _) = multispace0(i)?;
    let (i, imported_name) = parse_identifier(i)?;
    let (i, _) = multispace0(i)?;
    let (i, opt_val) = opt(tag("as"))(i)?;

    if opt_val != None {
        let (i, _) = multispace1(i)?;
        let (i, local_name) = parse_identifier(i)?;
        let (i, _) = multispace0(i)?;

        return Ok((
            i,
            ImportSpecifier {
                local: local_name,
                imported: imported_name,
            },
        ));
    }

    //todo!! add support for "as" keyword to rename import locally

    Ok((
        i,
        ImportSpecifier {
            local: imported_name.to_owned(),
            imported: imported_name,
        },
    ))
}
