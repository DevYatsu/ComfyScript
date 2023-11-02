use super::{
    ast::{
        identifier::{parse_identifier, Identifier},
        import::{ImportSource, ImportSpecifier},
        Statement, StatementKind,
    },
    expression::strings::parse_raw_string,
    reserved::DEFINED_FUNCTIONS,
};
use nom::{
    branch::alt,
    character::complete::{char, multispace0, multispace1, space0},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_import(i: &str) -> IResult<&str, Statement, ErrorTree<&str>> {
    let (i, _) = multispace1(i)?;

    let (i, asterisk) = char('*').opt().parse(i)?;

    let (i, specifiers) = if asterisk.is_none() {
        let (i, specifiers) = separated_list1(
            delimited(multispace0, char(','), multispace0),
            parse_import_specifier,
        )
        .cut()
        .parse(i)?;

        let (i, _) = preceded(multispace0, char(',')).opt().parse(i)?;
        let (i, _) = multispace1.cut().parse(i)?;

        (i, specifiers)
    } else {
        let (i, local) = import_as.opt().parse(i)?;
        let (i, _) = multispace0(i)?;

        let asterisk: Identifier = asterisk.unwrap().to_string().into();

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

    let (i, _) = tag("from").complete().cut().parse(i)?;
    let (i, _) = multispace1.cut().parse(i)?;

    let (i, source) = parse_raw_string
        .map(|s| ImportSource { value: s })
        .cut()
        .context("import source")
        .parse(i)?;

    let import_declaration: Statement = (source, specifiers).into();
    let (i, _) = space0(i)?;

    if i.is_empty() {
        return Ok((i, import_declaration));
    }

    let (i, _) = alt((tag("\n"), tag(","), tag(";"), tag("//").complete()))
        .peek()
        .context("unexpected")
        .cut()
        .parse(i)?;

    Ok((i, import_declaration))
}

fn parse_import_specifier(i: &str) -> IResult<&str, ImportSpecifier, ErrorTree<&str>> {
    let (i, imported_name) = parse_identifier.parse(i)?;
    let (i, local_name) = import_as.opt().parse(i)?;

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

fn import_as(i: &str) -> IResult<&str, Identifier, ErrorTree<&str>> {
    let (i, _) = multispace1(i)?;
    let (i, _) = tag("as").complete().parse(i)?;

    let (i, _) = multispace1.cut().parse(i)?;
    let (i, local_name) = parse_identifier
        .verify(|id| !DEFINED_FUNCTIONS.contains(&id.0.as_str()))
        .context("import specifier")
        .cut()
        .parse(i)?;

    return Ok((i, local_name));
}

impl Into<Statement> for (ImportSource, Vec<ImportSpecifier>) {
    fn into(self) -> Statement {
        Statement::with_kind(StatementKind::Import(self.0, self.1))
    }
}
