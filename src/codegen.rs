#[derive(Debug)]
pub struct Function {
    name: String,
    statements: Vec<Statement>,
}

impl Function {
    pub fn new(name: String, statements: Vec<Statement>) -> Self {
        Self { name, statements }
    }

    pub fn generate(&self) {
        println!(".globl {}\n{}:", self.name, self.name);
        for s in self.statements.iter() {
            s.generate();
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

impl Statement {
    pub fn generate(&self) {
        match self {
            Statement::Return(e) => {
                e.generate();
                println!("ret");
            }
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(usize),
    Minus(Box<Expression>),
    BinaryNot(Box<Expression>),
    LogicalNot(Box<Expression>),
}

impl Expression {
    pub fn generate(&self) {
        match self {
            Expression::Literal(i) => {
                println!("movl ${}, %eax", i);
            }
            Expression::Minus(e) => {
                e.generate();
                println!("neg %eax");
            }
            Expression::BinaryNot(e) => {
                e.generate();
                println!("not %eax");
            }
            Expression::LogicalNot(e) => {
                e.generate();
                println!("cmpl $0, %eax\nmovl $0, %eax\nsete %al");
            }
        }
    }
}

pub fn codegen(program: &[Function]) {
    for function in program {
        function.generate();
    }
}
