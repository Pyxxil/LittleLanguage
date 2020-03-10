#[derive(Debug)]
pub enum Expression {
    BooleanExpression,
    Assignment,
    ArrayLiteral(Vec<Expression>),
    StringLiteral(String),
    IntegerLiteral(i16),
    BooleanLiteral(bool),
    VariableDeclaration,
    Container(String, Vec<Expression>),
    Identifier(String),
}
