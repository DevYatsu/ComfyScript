use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::opt,
    error::VerboseError,
    multi::separated_list1,
    IResult,
};

use super::{
    ast::{identifier::Identifier, import::ImportSpecifier, ASTNode},
    primitive_values::strings::parse_string,
    utils::parse_identifier,
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
                local: Identifier { name: local_name },
                imported: Identifier {
                    name: imported_name,
                },
            },
        ));
    }

    //todo!! add support for "as" keyword to rename import locally

    Ok((
        i,
        ImportSpecifier {
            local: Identifier {
                name: imported_name.to_owned(),
            },
            imported: Identifier {
                name: imported_name,
            },
        },
    ))
}
