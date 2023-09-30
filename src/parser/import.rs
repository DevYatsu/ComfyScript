use nom::{
    bytes::complete::tag, character::complete::multispace0, combinator::opt, error::VerboseError,
    multi::separated_list1, IResult
};

use super::{
    ast::{identifier::Identifier, import::ImportSpecifier, ASTNode},
    utils::alpha_not_reserved,
    Span, primitive_values::strings::parse_string,
};

pub fn parse_import(i: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let start = i.len();
    let (i, _) = tag("import")(i)?;

    let (i, _) = multispace0(i)?;
    let (i, import_specifiers) = separated_list1(tag(","), parse_import_ids)(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = tag("from")(i)?;
    let (i, _) = multispace0(i)?;

    let (i, source) = parse_string(i)?; // todo! add expression Literal support instead
    let end = i.len();

    Ok((
        i,
        ASTNode::ImportDeclaration {
            specifiers: import_specifiers,
            source,
            start,
            end,
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
                local: Identifier {
                    name: local_name.fragment().to_string(),
                },
                imported: Identifier {
                    name: imported_name.fragment().to_string(),
                },
            },
        ));
    }

    //todo!! add support for "as" keyword to rename import locally

    Ok((
        input,
        ImportSpecifier {
            local: Identifier {
                name: imported_name.fragment().to_string(),
            },
            imported: Identifier {
                name: imported_name.fragment().to_string(),
            },
        },
    ))
}
