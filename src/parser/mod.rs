use std::fs::*;
use std::io::{Error, Read};
use std::path::PathBuf;

use nom::{
    branch::alt,
    bytes::complete::{escaped, escaped_transform, tag, take_while},
    character::complete::{anychar, char, one_of},
    combinator::{cut, map},
    error::{context, VerboseError},
    multi::{many0, separated_list},
    sequence::{delimited, pair, preceded, terminated},
    Err, IResult,
};

use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type ParseResult<'a, Out> = IResult<Span<'a>, Out, VerboseError<Span<'a>>>;

#[derive(Debug)]
enum Expression {
    VariableDeclaration(String),
    ContainerDeclaration(String, Vec<Expression>),
    FunctionDeclaration(String, Vec<Expression>, Vec<Expression>),
    StringLiteral(String),
    VariableAssignment(String, Vec<Expression>),
    Identifier(String),
}

fn parse_string(i: Span) -> ParseResult<String> {
    context(
        "parse_string",
        escaped_transform(anychar, '\\', one_of("\"n\\")),
    )(i)
}

fn string(i: Span) -> ParseResult<Expression> {
    map(
        preceded(
            context("Initial", char('\"')),
            cut(terminated(
                context("entering parsing", parse_string),
                context("Final", char('\"')),
            )),
        ),
        |s| Expression::StringLiteral(s.to_string()),
    )(i)
}

fn space(i: Span) -> ParseResult<Span> {
    let chars = " \t\r\n";

    take_while(move |ch| chars.contains(ch))(i)
}

fn identifier(i: Span) -> ParseResult<Span> {
    preceded(space, take_while(|ch: char| ch.is_alphabetic()))(i)
}

fn variable_declaration(i: Span) -> ParseResult<Expression> {
    preceded(
        preceded(space, tag("variable")),
        cut(map(preceded(space, identifier), |s| {
            Expression::VariableDeclaration(s.to_string())
        })),
    )(i)
}

fn assignment(i: Span) -> ParseResult<Vec<Expression>> {
    terminated(
        preceded(
            preceded(space, char('=')),
            preceded(
                space,
                alt((
                    map(identifier, |s| vec![Expression::Identifier(s.to_string())]),
                    map(string, |s| vec![s]),
                )),
            ),
        ),
        preceded(space, char(';')),
    )(i)
}

fn variable_definition(i: Span) -> ParseResult<Expression> {
    context(
        "Definition",
        terminated(
            map(pair(variable_declaration, assignment), |(ident, def)| {
                Expression::VariableAssignment(
                    match ident {
                        Expression::VariableDeclaration(ident) => ident,
                        _ => unreachable!(),
                    },
                    def,
                )
            }),
            char(';'),
        ),
    )(i)
}

fn container_declaration(i: Span) -> ParseResult<(String, Vec<Expression>)> {
    let identifier = identifier(i)?;
    let variables = delimited(
        preceded(space, tag("{")),
        many0(terminated(variable_declaration, char(';'))),
        preceded(space, tag("}")),
    )(identifier.0)?;
    Ok((variables.0, (identifier.1.to_string(), variables.1)))
}

fn scope(i: Span) -> ParseResult<Vec<Expression>> {
    context(
        "Scope",
        delimited(
            preceded(space, tag("{")),
            many0(preceded(
                space,
                alt((
                    variable_definition,
                    terminated(variable_declaration, preceded(space, char(';'))),
                    string,
                )),
            )),
            preceded(space, tag("}")),
        ),
    )(i)
}

fn arguments(i: Span) -> ParseResult<Vec<Expression>> {
    delimited(
        preceded(space, tag("(")),
        separated_list(
            preceded(space, tag(",")),
            preceded(space, variable_declaration),
        ),
        preceded(space, tag(")")),
    )(i)
}

fn function_declaration(i: Span) -> ParseResult<(String, Vec<Expression>, Vec<Expression>)> {
    let identifier = identifier(i)?;
    let arguments = arguments(identifier.0)?;
    let body = scope(arguments.0)?;
    Ok((body.0, (identifier.1.to_string(), arguments.1, body.1)))
}

fn parse(content: &str) -> ParseResult<Vec<Expression>> {
    many0(preceded(
        space,
        alt((
            map(
                preceded(tag("container"), cut(container_declaration)),
                |(ident, variables)| {
                    let expr = Expression::ContainerDeclaration(ident, variables);
                    println!("{:#?}", expr);
                    expr
                },
            ),
            map(
                preceded(tag("function"), cut(function_declaration)),
                |(name, args, body)| {
                    let expr = Expression::FunctionDeclaration(name, args, body);
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
