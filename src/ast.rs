#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Map(Vec<(String, Expr)>),
    String(String),
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
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Boolean(bool),
    Array(Vec<Expr>),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    IndexAssign {
        object: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
    },
    Dot {
        // Dot notation: obj.field
        object: Box<Expr>,
        field: String,
    },
    DotAssign {
        // Dot assignment: obj.field = value
        object: Box<Expr>,
        field: String,
        value: Box<Expr>,
    },
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
    For {
        // <-- ADD THIS!
        variable: String,
        iterable: Box<Expr>,
        body: Box<Stmt>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Return {
        value: Option<Expr>,
    },
}
pub struct Program {
    pub statements: Vec<Stmt>,
}
