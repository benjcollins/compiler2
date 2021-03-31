#[derive(Debug)]
pub enum Statement<'a> {
    Assignment {
        idents: Vec<&'a str>,
        expr: Expr<'a>,
    },
    Declaration {
        decls: Vec<Declaration<'a>>,
        expr: Option<Expr<'a>>,
    },
    Return(Expr<'a>),
}

#[derive(Debug)]
pub struct Declaration<'a> {
    pub idents: Vec<&'a str>,
    pub ty: Option<Type<'a>>,
}

#[derive(Debug)]
pub struct Type<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub enum Expr<'a> {
    IntLiteral(&'a str),
    Tuple(Vec<Expr<'a>>),
    Ident(&'a str),
}

#[derive(Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub params: Vec<Declaration<'a>>,
    pub rets: Vec<Type<'a>>,
    pub statements: Vec<Statement<'a>>,
}