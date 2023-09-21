pub fn parse_for(input: &str) -> IResult<&str, Assignment, VerboseError<&str>> {
    let (input, keyword) = parse_variable_keyword(input)?;

    let (input, _) = multispace0(input)?;
    let (input, name) = alphanumeric1(input)?;

    if VariableKeyword::equals_any(name) {
        let e = VerboseError::add_context(
            input,
            "Invalid variable name!",
            VerboseError {
                errors: vec![(
                    "Invalid variable name!",
                    VerboseErrorKind::Context("Invalid variable name!"),
                )],
            },
        );
        return Err(nom::Err::Error(e));
    };

    let (input, _) = multispace0(input)?;

    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = parse_bool(input)?;

    let result = (input, Assignment::new(keyword, name.to_owned(), value));

    Ok(result)
}