#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Variable(String),
    Assign(String, Box<Expr>),
    Binary {
        left: Box<Expr>,
        operator: BinOp,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: LogicalOp,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        right: Box<Expr>,
    },
    Call {
        callee: String,
        arguments: Vec<Expr>,
    },
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    EqualEqual,
    BangEqual,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        initializer: Option<Expr>,
    },
    Print(Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
