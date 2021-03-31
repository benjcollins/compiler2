use self::ast::{Declaration, Expr, Function, Statement, Type};

pub mod ast;

struct Parser<'a> {
    line: u32,
    column: u32,
    source: &'a str,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Precedence {
    Expr,
    Tuple
}

impl<'a> Parser<'a> {
    fn consume_char(&mut self) {
        let mut chars = self.source.chars();
        match chars.next() {
            Some('\n') => {
                self.line += 1;
                self.column = 1;
            },
            Some(_) => {
                self.column += 1;
            }
            None => {}
        }
        self.source = chars.as_str();
    }
    fn peek_char(&self) -> Option<char> {
        self.source.chars().next()
    }
    fn consume_whitespace(&mut self) {
        while self.peek_char().map(|ch| ch.is_whitespace()).unwrap_or(false) {
            self.consume_char()
        }
    }
    fn try_consume_symbol(&mut self, symbol: &str) -> bool {
        if self.peek_symbol(symbol) {
            for _ in symbol.chars() {
                self.consume_char();
            }
            true
        } else {
            false
        }
    }
    fn consume_symbol(&mut self, symbol: &str) -> Result<(), ()> {
        if self.try_consume_symbol(symbol) {
            Ok(())
        } else {
            Err(())
        }
    }
    fn peek_symbol(&mut self, symbol: &str) -> bool {
        self.consume_whitespace();
        self.source.starts_with(symbol)
    }
    fn consume_ident(&mut self) -> Option<&'a str> {
        self.consume_whitespace();
        if self.peek_char()?.is_alphabetic() {
            let source = self.source;
            let mut length = 0;
            loop {
                let ch = match self.peek_char() {
                    Some(ch) if ch.is_alphanumeric() => ch,
                    _ => break
                };
                length += ch.len_utf8();
                self.consume_char();
            }
            Some(&source[..length])
        } else {
            None
        }
    }
    fn consume_int(&mut self) -> Option<&'a str> {
        self.consume_whitespace();
        let source = self.source;
        let mut length = 0;
        loop {
            let ch = match self.peek_char() {
                Some(ch) if ch.is_numeric() => ch,
                _ => break,
            };
            length += ch.len_utf8();
            self.consume_char();
        }
        if length > 0 {
            Some(&source[..length])
        } else {
            None
        }
    }
    fn parse_idents(&mut self, first: &'a str) -> Vec<&'a str> {
        let mut idents = vec![first];
        while self.try_consume_symbol(",") {
            idents.push(self.consume_ident().unwrap());
        }
        idents
    }
    fn parse_type(&mut self) -> Type<'a> {
        Type { ident: self.consume_ident().unwrap() }
    }
    fn parse_tuple(&mut self, left: &mut Expr<'a>, prec: Precedence) -> bool {
        if self.peek_symbol(",") && prec >= Precedence::Tuple {
            let mut tuple = vec![std::mem::replace(left, Expr::Tuple(vec![]))];
            while self.try_consume_symbol(",") {
                tuple.push(self.parse_expr(Precedence::Expr))
            }
            *left = Expr::Tuple(tuple);
            true
        } else {
            false
        }
    }
    fn parse_expr(&mut self, prec: Precedence) -> Expr<'a> {
        let mut left = self.consume_int().map(|int| Expr::IntLiteral(int))
            .or_else(|| self.consume_ident().map(|ident| Expr::Ident(ident)))
            .unwrap();

        loop {
            if !self.parse_tuple(&mut left, prec) {
                break;
            }
        }
        left
    }
    fn parse_decl(&mut self) -> Declaration<'a> {
        let ident = self.consume_ident().unwrap();
        let idents = self.parse_idents(ident);
        if !self.try_consume_symbol(":") {
            panic!()
        }
        let ty = Some(self.parse_type());
        Declaration { idents, ty }
    }
    fn parse_decls(&mut self, first: Declaration<'a>) -> Vec<Declaration<'a>> {
        let mut decls = vec![first];
        while self.try_consume_symbol(",") {
            decls.push(self.parse_decl());
        }
        decls
    }
    fn parse_statement(&mut self) -> Statement<'a> {
        let ident = self.consume_ident().unwrap();
        if self.try_consume_symbol("(") {
            todo!()
        }
        if ident == "return" {
            let expr = self.parse_expr(Precedence::Tuple);
            return Statement::Return(expr)
        }
        let idents = self.parse_idents(ident);
        if self.try_consume_symbol("=") {
            let expr = self.parse_expr(Precedence::Tuple);
            return Statement::Assignment { idents, expr }
        }
        if self.try_consume_symbol(":") {
            let decls = if self.peek_symbol("=") {
                idents.iter().map(|ident| Declaration { idents: vec![ident], ty: None }).collect()
            } else {
                let ty = Some(self.parse_type());
                self.parse_decls(Declaration { idents, ty })
            };
            let expr = if self.try_consume_symbol("=") {
                Some(self.parse_expr(Precedence::Tuple))
            } else {
                None
            };
            return Statement::Declaration { decls, expr }
        }
        panic!()
    }
    fn parse_function(&mut self) -> Result<Function<'a>, ()> {
        self.consume_symbol("fn")?;
        let name = self.consume_ident().unwrap();
        self.consume_symbol("(")?;
        let decl = self.parse_decl();
        let params = self.parse_decls(decl);
        self.consume_symbol(")")?;
        let rets = vec![];
        if self.try_consume_symbol("->") {
            rets.push(self.parse_type());
            while self.try_consume_symbol(",") {
                rets.push(self.parse_type())
            }
        }
        self.consume_symbol("{")?;

        let mut statements = vec![];
        while !self.try_consume_symbol("}") {
            statements.push(self.parse_statement());
        }
        Ok(Function { name, params, rets, statements })
    }
}

pub fn parse<'a>(source: &'a str) -> Function<'a> {
    let mut parser = Parser { source, line: 1, column: 1 };
    parser.parse_function().unwrap()
}