use custom_error::custom_error;

custom_error! {pub ParsingError
    Custom{string: String} = "{string}",
    ImportError{source: ImportError} = "{source}",
}
custom_error! {pub ImportError
    Custom{string: String} = "{string}",
    ExpectedSpecifier{code: String, line: String} = "Import Error: Expected a valid import specifier \n At {code} \n line {line}",
    ExpectedImportSrc{code: String, line: String} = "Import Error: Expected a valid import source \n At {code} \n line {line}",
}
