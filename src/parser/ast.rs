pub type Program = Vec<Stmt>;
pub type BlockStmt = Vec<Stmt>;

#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    Let(Ident, Expr, LLVMExpressionType),
    Assignment(Ident, Expr),
    Func {
        name: String,
        distributed: bool,
        params: Vec<Ident>,
        param_types: Vec<LLVMExpressionType>,
        return_type: LLVMExpressionType,
        body: Program,
    },
    Return(Expr),
    Expr(Expr),
    Blank,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    Prefix(Prefix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),
    If {
        cond: Box<Expr>,
        consequence: Program,
        alternative: Option<Program>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
   // Array(Vec<Expr>),
   // Hash(Vec<(Literal, Expr)>),
   /* Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },*/
}


#[derive(PartialEq, Clone, Debug)]
pub enum LLVMExpressionType {
    Integer,
//    String(u32),
    Boolean,
    Null,
 //   Array(Box<LLVMExpressionType>, u32),
    Call,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Int(i64),
    Bool(bool),
   // String(String),
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

#[derive(PartialEq, Debug, Clone)]
pub enum Prefix {
    Plus,
    Minus,
    Not
}

#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    Plus,
    Minus,
    Equal,
    NotEqual,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

