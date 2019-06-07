#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
}

pub type Identifier = String;

#[derive(Debug)]
pub struct Program {
    pub funs: Vec<Function>,
}

impl Program {
    pub fn new(funs: Vec<Function>) -> Self {
        Self { funs }
    }
}


#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<(Type, Identifier)>,
    pub statements: Option<Vec<Statement>>,
}

impl Function {
    pub fn new(
        name: String,
        args: Vec<(Type, Identifier)>,
        statements: Option<Vec<Statement>>,
    ) -> Self {
        Self {
            name,
            statements,
            args,
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Declaration(Type, Identifier, Option<Expression>),
    Return(Expression), // TODO: empty return
    Expression(Option<Expression>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Compound(Vec<Statement>),
    For(
        Option<Expression>,
        Expression,
        Option<Expression>,
        Box<Statement>,
    ),
    ForDecl(
        Box<Statement>,
        Expression,
        Option<Expression>,
        Box<Statement>,
    ),
    While(Expression, Box<Statement>),
    Do(Box<Statement>, Expression),
    Break,
    Continue,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Literal(usize),
    Minus(Box<Expression>),
    BinaryNot(Box<Expression>),
    LogicalNot(Box<Expression>),
    PreIncrement(Identifier),
    PreDecrement(Identifier),
    PostIncrement(Identifier),
    PostDecrement(Identifier),
    Subtract(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    LessThanOrEqual(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    GreaterThanOrEqual(Box<Expression>, Box<Expression>),
    Assignment(Identifier, Box<Expression>),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
    FunCall(Identifier, Vec<Expression>),
}
