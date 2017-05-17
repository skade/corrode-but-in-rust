#[derive(Clone, Debug)]
pub enum Expr {
    Number(isize),
    Op(Box<Expr>, String, Box<Expr>),
    Ref(Ident),
    Do(Vec<Expr>, Option<Vec<Statement>>),
    Parens(Vec<Expr>),
    Case(Box<Expr>, Vec<CaseCond>),
    Let,
    Span(Vec<Expr>),
    Vector(Vec<Expr>),
    Operator(String),
    Record(Vec<(Ident, Expr)>),
    Lambda,
    Str(String),
    Char(String),
    Error,
    Dummy,
}

#[derive(Clone, Debug)]
pub enum CaseCond {
    Matching(Vec<Pat>, Vec<(Vec<Expr>, Expr)>),
    Direct(Vec<Pat>, Vec<Expr>),
    Where,
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Clone, Debug)]
pub enum Statement {
    // Name, Inner Types, Deriving IDs
    Data(Ident, Vec<Vec<Ty>>, Vec<Ident>),
    Class,
    Instance,
    Import,
    Newtype,
    Pipelist,
    GuardAssign,
    Assign(Pat, Vec<Pat>, Expr),
    Typedef(Ident),
    Prototype(Ident, Vec<Ty>),

    // TODO remove this
    Dummy,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub name: Ident,
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum Ty {
    Span(Vec<Ty>),
    Where(Box<Ty>, Box<Ty>),
    Pair(Box<Ty>, Box<Ty>),
    Not(Box<Ty>),
    Ref(Ident),
    Tuple(Vec<Ty>),
    Brackets(Box<Ty>),
    RecordTODO,
    EmptyParen,
}

#[derive(Clone, Debug)]
pub enum Pat {
    Span(Vec<Pat>),
    Arrow(Ident, Box<Pat>),
    Not(Box<Pat>),
    Ref(Ident),
    Tuple(Vec<Pat>),
    Brackets(Vec<Pat>),
    RecordTODO,
    Str(String),
    Char(String),
    Num(isize),
    EmptyParen,
    Dummy,
}

#[derive(Clone, Debug)]
pub struct Ident(pub String);