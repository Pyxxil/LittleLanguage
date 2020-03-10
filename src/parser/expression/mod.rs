#[derive(Debug)]
pub enum Expression {
    VariableDeclaration(String),
    ContainerDeclaration(String, Vec<Expression>),
    FunctionDeclaration(String, Vec<Expression>, Vec<Expression>),
    StringLiteral(String),
    VariableAssignment(String, Vec<Expression>),
    Identifier(String),
}
