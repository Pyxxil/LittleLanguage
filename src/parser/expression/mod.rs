#[derive(Debug)]
pub struct Function {
    name: String,
    arguments: Vec<Expression>,
    body: Vec<Expression>,
}

impl Function {
    pub fn new(name: String, arguments: Vec<Expression>, body: Vec<Expression>) -> Self {
        Self {
            name,
            arguments,
            body,
        }
    }
}

#[derive(Debug)]
pub struct Container {
    name: String,
    variables: Vec<Expression>,
}

impl Container {
    pub fn new(name: String, variables: Vec<Expression>) -> Self {
        Self { name, variables }
    }
}

#[derive(Debug)]
pub enum Expression {
    VariableDeclaration(String, String),
    ContainerDeclaration(Container),
    FunctionDeclaration(Function),
    StringLiteral(String),
    VariableAssignment(String, String, Option<Vec<Expression>>),
    Identifier(String),
    Comment(String),
    IntegerLiteral(String),
}

impl Expression {
    pub fn assign_from_declaration(expr: Self, def: Option<Vec<Self>>) -> Self {
        let (ty, ident) = match expr {
            Self::VariableDeclaration(ty, ident) => (ty, ident),
            _ => unreachable!(),
        };

        Self::VariableAssignment(ty, ident, def)
    }
}
