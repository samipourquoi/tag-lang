use nom::{
    IResult,
    multi::{many0},
    combinator::{opt},
    bytes::complete::{tag},
    character::complete::{anychar}
};

#[derive(Debug, PartialEq)]
struct Command {
    value: String
}

type ParseResult<'a, T> = IResult<&'a str, T>;

fn parse_command(input: &str) -> ParseResult<Command> {
    let (input, _) = opt(tag("/"))(input)?;
    let (input, command) = many0(anychar)(input)?;

    Ok((input, Command {
        value: command.iter().collect()
    }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn command() {
        let result = parse_command("/say hello world");
        assert_eq!(result, Ok(("", "say hello world")))
    }
}
