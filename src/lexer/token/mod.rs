#[derive(Debug)]
pub enum Token {
    If(Location),
    Else(Location),
    Variable(Location),
    For(Location),
    Function(Location),
    Container(Location),

    Identifier(Location, String),

    True(Location),
    False(Location),
    NumberLiteral(Location, String),
    StringLiteral(Location, String),

    Semicolon(Location),
    Comma(Location),
    OpenBracket(Location),
    CloseBracket(Location),
    OpenParentheses(Location),
    CloseParentheses(Location),
    OpenCurlyBrace(Location),
    CloseCurlyBrace(Location),
    Dot(Location),

    Assignment(Location),

    Equality(Location),
    GreaterThan(Location),
    GreaterEqualTo(Location),
    LessThan(Location),
    LessEqualTo(Location),
    LogicalNot(Location),
    LogicalNotEqualTo(Location),
    LogicalOr(Location),
    LogicalAnd(Location),

    BitwiseOr(Location),
    BitwiseAnd(Location),
    BitwiseNot(Location),

    SubtractEqual(Location),
    AddEqual(Location),
    MultiplyEqual(Location),
    DivideEqual(Location),

    Subtraction(Location),
    Addition(Location),
    Multiply(Location),
    Divide(Location),
}

#[derive(Debug)]
pub struct Location {
    column: usize,
    line: usize,
}

impl Location {
    pub fn new(column: usize, line: usize) -> Self {
        Self { column, line }
    }
}
