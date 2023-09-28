use nom::{
    character::complete::{alphanumeric1, multispace0},
    combinator::opt,
    error::VerboseError,
    multi::separated_list1,
    IResult,
};
use nom_supreme::tag::complete::tag;

use super::{
    ast::{identifier::Identifier, import::ImportSpecifier, ASTNode, Expression},
    strings::parse_string,
    utils::alpha_not_reserved,
};

pub fn parse_import(i: &str) -> IResult<&str, ASTNode, VerboseError<&str>> {
    let (input, _) = tag("import")(i)?;

    let (input, _) = multispace0(input)?;
    let (input, import_specifiers) = separated_list1(tag(","), parse_import_ids)(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = tag("from")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, source) = parse_string(input)?; // todo! add expression Literal support instead

    Ok((
        input,
        ASTNode::ImportDeclaration {
            specifiers: import_specifiers,
            source,
        },
    ))
}

fn parse_import_ids(input: &str) -> IResult<&str, ImportSpecifier, VerboseError<&str>> {
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
                    name: local_name.to_owned(),
                },
                imported: Identifier {
                    name: imported_name.to_owned(),
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
                name: imported_name.to_owned(),
            },
        },
    ))
}
