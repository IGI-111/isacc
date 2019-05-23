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
    Subtract(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn generate(&self) {
        match self {
            Expression::Literal(i) => {
                println!("movq ${}, %rax", i);
            }
            Expression::Minus(e) => {
                e.generate();
                println!("neg %rax");
            }
            Expression::BinaryNot(e) => {
                e.generate();
                println!("not %rax");
            }
            Expression::LogicalNot(e) => {
                e.generate();
                println!("cmpl $0, %rax\nmovq $0, %rax\nsete %al");
            }
            Expression::Subtract(e1, e2) => {
                e1.generate();
                println!("pushq %rax");
                e2.generate();
                println!("popq %rcx\nsubq %rcx, %rax");
            }
            Expression::Add(e1, e2) => {
                e1.generate();
                println!("pushq %rax");
                e2.generate();
                println!("popq %rcx\naddq %rcx, %rax");
            }
            Expression::Multiply(e1, e2) => {
                e1.generate();
                println!("pushq %rax");
                e2.generate();
                println!("popq %rcx\nimulq %rcx, %rax");
            }
            Expression::Divide(e1, e2) => {
                e1.generate();
                println!("pushq %rax");
                e2.generate();
                println!("movq $0, %rdx\nmovq %rax, %rcx\npopq %rax\nidivq %rcx");
            }
        }
    }
}

pub fn codegen(program: &[Function]) {
    for function in program {
        function.generate();
    }
}
