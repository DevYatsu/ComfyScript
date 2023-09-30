use nom::{
    bytes::complete::tag, character::complete::multispace0, combinator::opt, error::VerboseError,
    multi::separated_list1, IResult,
};

use super::{
    ast::{identifier::Identifier, import::ImportSpecifier, ASTNode},
    primitive_values::strings::parse_string,
    utils::alpha_not_reserved,
    Span,
};

pub fn parse_import(i: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (i, _) = tag("import")(i)?;

    let (i, _) = multispace0(i)?;
    let (i, import_specifiers) = separated_list1(tag(","), parse_import_ids)(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = tag("from")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, source) = parse_string(i)?; // todo! add expression Literal support instead

    Ok((
        i,
        ASTNode::ImportDeclaration {
            specifiers: import_specifiers,
            source,
        },
    ))
}

fn parse_import_ids(input: Span) -> IResult<Span, ImportSpecifier, VerboseError<Span>> {
    let (input, _) = multispace0(input)?;
    let (input, imported_name) = alpha_not_reserved(input)?;
    let (input, _) = multispace0(input)?;
    let (input, opt_val) = opt(tag("as"))(input)?;

    if opt_val != None {
        let (input, _) = multispace0(input)?;
        let (input, local_name) = alpha_not_reserved(input)?;

        return Ok((
            input,
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
        input,
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
