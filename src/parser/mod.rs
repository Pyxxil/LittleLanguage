use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub mod expression;
use expression::{Container, Expression, Function};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::{alphanumeric1 as alphanumeric, char, multispace1},
    combinator::{cut, map, opt, value, verify},
    error::{context, VerboseError},
    multi::{fold_many0, many0, separated_list},
    sequence::{delimited, pair, preceded, terminated, tuple},
    Err, IResult,
};

use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type ParseResult<'a, Out> = IResult<Span<'a>, Out, VerboseError<Span<'a>>>;

fn parse_escaped_char(input: Span) -> ParseResult<char> {
    preceded(
        char('\\'),
        // `alt` tries each each parser in sequence, returning the result of
        // the first successful match
        alt((
            // The `value` parser returns a fixed value (the first argument) if its
            // parser (the second argument) succeeds. In these cases, it looks for
            // the marker characters (n, r, t, etc) and returns the matching
            // character (\n, \r, \t, etc).
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\\', char('\\')),
            value('/', char('/')),
            value('"', char('"')),
        )),
    )(input)
}

/// Parse a backslash, followed by any amount of whitespace. This is used later
/// to discard any escaped whitespace.
fn parse_escaped_whitespace(i: Span) -> ParseResult<Span> {
    preceded(char('\\'), multispace1)(i)
}

/// Parse a non-empty block of text that doesn't include \ or "
fn parse_literal(i: Span) -> ParseResult<Span> {
    // `is_not` parses a string of 0 or more characters that aren't one of the
    // given characters.
    let not_quote_slash = is_not("\"\\");

    // `verify` runs a parser, then runs a verification function on the output of
    // the parser. The verification function accepts out output only if it
    // returns true. In this case, we want to ensure that the output of is_not
    // is non-empty.
    verify(not_quote_slash, |s: &Span| !s.to_string().is_empty())(i)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(Span<'a>),
    EscapedChar(char),
    EscapedWS,
}

/// Combine `parse_literal`, `parse_escaped_whitespace`, and `parse_escaped_char`
/// into a `StringFragment`.
fn parse_fragment(input: Span) -> ParseResult<StringFragment> {
    alt((
        // The `map` combinator runs a parser, then applies a function to the output
        // of that parser.
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
        value(StringFragment::EscapedWS, parse_escaped_whitespace),
    ))(input)
}

fn parse_string(input: Span) -> ParseResult<Expression> {
    // fold_many0 is the equivalent of iterator::fold. It runs a parser in a loop,
    // and for each output value, calls a folding function on each output value.
    let build_string = fold_many0(
        // Our parser function– parses a single string fragment
        parse_fragment,
        // Our init value, an empty string
        String::new(),
        // Our folding function. For each fragment, append the fragment to the
        // string.
        |mut string, fragment| {
            match fragment {
                StringFragment::Literal(s) => string.push_str(s.to_string().as_str()),
                StringFragment::EscapedChar(c) => string.push(c),
                StringFragment::EscapedWS => {}
            }
            string
        },
    );

    // Finally, parse the string. Note that, if `build_string` could accept a raw
    // " character, the closing delimiter " would never match. When using
    // `delimited` with a looping parser (like fold_many0), be sure that the
    // loop won't accidentally match your closing delimiter!
    delimited(
        char('"'),
        map(build_string, Expression::StringLiteral),
        char('"'),
    )(input)
}

fn space(input: Span) -> ParseResult<Span> {
    let chars = " \t\r\n";

    take_while(move |ch| chars.contains(ch))(input)
}

fn identifier(input: Span) -> ParseResult<Span> {
    context(
        "Identifier",
        preceded(
            space,
            verify(alphanumeric, |ch: &Span| {
                ch.to_string().chars().next().unwrap().is_alphabetic()
            }),
        ),
    )(input)
}

fn integer_literal(input: Span) -> ParseResult<Expression> {
    preceded(
        space,
        map(take_while(|ch: char| ch.is_digit(10)), |s: Span| {
            Expression::IntegerLiteral(s.to_string())
        }),
    )(input)
}

fn r#type(input: Span) -> ParseResult<Span> {
    preceded(
        space,
        alt((
            tag("integer"),
            tag("string"),
            tag("boolean"),
            tag("character"),
            identifier,
        )),
    )(input)
}

fn variable_declaration(input: Span) -> ParseResult<Expression> {
    context(
        "Declaration",
        map(pair(r#type, identifier), |(r#type, ident)| {
            let variable = Expression::VariableDeclaration(r#type.to_string(), ident.to_string());
            println!("Variable: {:#?}", variable);
            variable
        }),
    )(input)
}

fn assignment(input: Span) -> ParseResult<Vec<Expression>> {
    context(
        "Assignment",
        preceded(
            preceded(space, char('=')),
            cut(preceded(
                space,
                alt((
                    map(integer_literal, |s| {
                        let integer = vec![s];
                        println!("{:#?}", integer);
                        integer
                    }),
                    map(parse_string, |s| {
                        let string = vec![s];
                        println!("{:#?}", string);
                        string
                    }),
                    map(identifier, |s| {
                        let ident = vec![Expression::Identifier(s.to_string())];
                        println!("{:#?}", ident);
                        ident
                    }),
                )),
            )),
        ),
    )(input)
}

fn variable_definition(input: Span) -> ParseResult<Expression> {
    context(
        "Definition",
        terminated(
            map(
                pair(
                    variable_declaration,
                    map(opt(assignment), |s| {
                        println!("{:#?}", s);
                        s
                    }),
                ),
                |(ident, def)| Expression::assign_from_declaration(ident, def),
            ),
            preceded(space, tag(";")),
        ),
    )(input)
}

fn container_declaration(input: Span) -> ParseResult<Container> {
    context(
        "Container Declaration",
        map(
            pair(
                identifier,
                delimited(
                    preceded(space, char('{')),
                    many0(terminated(variable_declaration, char(';'))),
                    preceded(space, char('}')),
                ),
            ),
            |(identifier, variables)| Container::new(identifier.to_string(), variables),
        ),
    )(input)
}

fn scope(input: Span) -> ParseResult<Vec<Expression>> {
    context(
        "Scope",
        delimited(
            preceded(space, char('{')),
            many0(alt((
                map(comment, |s| Expression::Comment(s.to_string())),
                map(variable_definition, |def| {
                    println!("{:#?}", def);
                    def
                }),
                parse_string,
            ))),
            preceded(space, char('}')),
        ),
    )(input)
}

fn arguments(input: Span) -> ParseResult<Vec<Expression>> {
    context(
        "Arguments",
        delimited(
            preceded(space, char('(')),
            separated_list(
                preceded(space, char(',')),
                preceded(space, variable_declaration),
            ),
            preceded(space, char(')')),
        ),
    )(input)
}

fn function_declaration(input: Span) -> ParseResult<Function> {
    context(
        "Function Declaration",
        map(
            tuple((identifier, arguments, scope)),
            |(identifier, arguments, body)| Function::new(identifier.to_string(), arguments, body),
        ),
    )(input)
}

fn comment(input: Span) -> ParseResult<Span> {
    context(
        "Comment",
        preceded(preceded(space, tag("//")), cut(take_while(|ch| ch != '\n'))),
    )(input)
}

fn parse(content: &str) -> ParseResult<Vec<Expression>> {
    many0(preceded(
        space,
        alt((
            map(comment, |s| Expression::Comment(s.to_string())),
            map(
                preceded(tag("container"), cut(container_declaration)),
                |container| {
                    let expr = Expression::ContainerDeclaration(container);
                    println!("{:#?}", expr);
                    expr
                },
            ),
            map(
                preceded(tag("function"), cut(function_declaration)),
                |func| {
                    let expr = Expression::FunctionDeclaration(func);
                    println!("{:#?}", expr);
                    expr
                },
            ),
        )),
    ))(Span::new(content))
}

#[derive(Debug)]
pub struct Parser {
    expressions: Vec<Expression>,
}

impl<'a> Parser {
    pub fn from_content(content: &str) -> Result<Self, String> {
        match parse(content) {
            Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(e
                .errors
                .into_iter()
                .map(|error| format!("{:#?}", error).lines().collect::<String>())
                .collect()),
            Ok(tree) => Ok(Self {
                expressions: tree.1,
            }),
            _ => Err(String::from("Unknown Error")),
        }
    }

    pub fn from_file(file: PathBuf) -> Result<Self, String> {
        let err = format!("Unable to open file {:#?}", file);
        match File::open(file) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                Self::from_content(&content)
            }
            Err(_) => Err(err),
        }
    }
}
