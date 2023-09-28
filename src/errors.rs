use custom_error::custom_error;

custom_error! {pub ParsingError
    Custom{string: String} = "{string}",
}
